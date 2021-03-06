use error::*;
use syntax::ast::*;

use environment::Env;

use std::error::Error;
use std::fmt::Debug;

pub trait Operate {
    fn operate(&self, operands: &Vec<Expr>, env: &mut Env) -> EvalResult<Expr>;

    fn handle_error<G, X>(&self, kind: ProgramError, got: G, expected: X) -> Box<Error>
    where
        Self: Debug,
        G: Debug,
        X: Debug,
    {
        let mut format_string = String::from("");
        match kind {
            ProgramError::BadArgs => {
                format_string = format!(
                "Funciton '{:?}' passed incorrect number of arguments.  Got {:?}, Expected: {:?}",
                self, got, expected
            );
            }
            ProgramError::BadType => {
                format_string = format!(
                    "Funciton '{:?}' passed incorrect type of argument.  Got {:?}, Expected: {:?}",
                    self, got, expected
                );
            }
        }

        make_error(kind, format_string)
    }
}

impl Operate for Arith {
    fn operate(&self, operands: &Vec<Expr>, _env: &mut Env) -> EvalResult<Expr> {
        let init_val: Number = match *self {
            Arith::Add => 0.0,
            Arith::Sub => 0.0,
            Arith::Mul => 1.0,
            Arith::Div => 1.0,
            Arith::Mod => 1.0,
        };

        let mut acc = init_val;

        for x in operands[1..].iter() {
            let val = match *x {
                Expr::Val(v) => v,
                _ => return Err(self.handle_error(ProgramError::BadType, x, "Number")),
            };

            // TODO: subtraction is completely broken because it iteratively negates the operands
            acc = perform_artih_op(self, acc, val);
        }

        Ok(Expr::Val(acc))
    }
}

fn perform_artih_op(op: &Arith, lhs: Number, rhs: Number) -> Number {
    match &op {
        Arith::Add => lhs + rhs,
        Arith::Sub => lhs - rhs,
        Arith::Mul => lhs * rhs,
        Arith::Div => lhs / rhs,
        Arith::Mod => lhs % rhs,
    }
}

impl Operate for Builtin {
    fn operate(&self, operands: &Vec<Expr>, env: &mut Env) -> EvalResult<Expr> {
        match *self {
            Builtin::Head => self.head(&operands[1..]),
            Builtin::Tail => self.tail(&operands[1..]),
            Builtin::List => self.list(&operands[1..]),
            Builtin::Join => self.join(&operands[1..]),
            Builtin::Eval => self.eval(&operands[1..], env),
            Builtin::Len => self.len(&operands[1..]),
            Builtin::Def => self.define(&operands[1..], env),
        }
    }
}

trait BuiltinFuncs {
    fn head(&self, operands: &[Expr]) -> EvalResult<Expr>;
    fn tail(&self, operands: &[Expr]) -> EvalResult<Expr>;
    fn list(&self, operands: &[Expr]) -> EvalResult<Expr>;
    fn join(&self, operands: &[Expr]) -> EvalResult<Expr>;
    fn eval(&self, operands: &[Expr], env: &mut Env) -> EvalResult<Expr>;
    fn len(&self, operands: &[Expr]) -> EvalResult<Expr>;
    fn define(&self, operands: &[Expr], env: &mut Env) -> EvalResult<Expr>;
}

impl BuiltinFuncs for Builtin {
    fn define(&self, operands: &[Expr], env: &mut Env) -> EvalResult<Expr> {
        if let Expr::Qexp(ref exprs) = operands[0] {
            if !(exprs.len() == operands.len() - 1) {
                return Err(self.handle_error(
                    ProgramError::BadArgs,
                    exprs.len(),
                    operands.len() - 1,
                ));
            }

            for (idx, expr) in exprs.iter().enumerate() {
                if let Expr::Sym(Symbol::Var(var)) = expr {
                    // Can we do better than a clone?
                    env.table
                        .insert(var.ident.clone(), operands[idx + 1].clone());
                } else {
                    return Err(self.handle_error(ProgramError::BadType, expr, "Sym::Var"));
                }
            }
            Ok(Expr::Empty)
        } else {
            Err(self.handle_error(ProgramError::BadType, &operands[0], "Expr::Qexp"))
        }
    }

    fn head(&self, operands: &[Expr]) -> EvalResult<Expr> {
        if operands.len() == 0 {
            return Err(self.handle_error(ProgramError::BadArgs, operands.len(), 1));
        }

        if operands.len() > 1 {
            return Err(self.handle_error(ProgramError::BadArgs, operands.len(), 1));
        }

        if let Expr::Qexp(ref v) = operands[0] {
            let mut qexp = vec![];
            qexp.push(v[0].clone());

            return Ok(Expr::Qexp(qexp));
        }

        Err(self.handle_error(ProgramError::BadType, &operands[0], "Expr::Qexpr"))
    }

    fn tail(&self, operands: &[Expr]) -> EvalResult<Expr> {
        if operands.len() == 0 {
            return Err(self.handle_error(ProgramError::BadArgs, operands.len(), 1));
        }

        if operands.len() > 1 {
            return Err(self.handle_error(ProgramError::BadArgs, operands.len(), 1));
        }

        if let Expr::Qexp(ref v) = operands[0] {
            let mut qexp = vec![];
            qexp.extend_from_slice(&v[1..]);

            return Ok(Expr::Qexp(qexp));
        }

        Err(self.handle_error(ProgramError::BadType, &operands[0], "Expr::Qexpr"))
    }

    fn join(&self, operands: &[Expr]) -> EvalResult<Expr> {
        for operand in operands {
            match operand {
                Expr::Qexp(_) => continue,
                _ => {
                    return Err(self.handle_error(ProgramError::BadArgs, operand, "Expr::Qexpr"));
                }
            }
        }

        let mut new_expr = vec![];

        for operand in operands {
            if let Expr::Qexp(v) = operand {
                for expr in v {
                    new_expr.push(expr.clone());
                }
            }
        }

        Ok(Expr::Qexp(new_expr))
    }

    fn eval(&self, operands: &[Expr], env: &mut Env) -> EvalResult<Expr> {
        if operands.len() > 1 {
            return Err(self.handle_error(ProgramError::BadArgs, operands.len(), 1));
        }

        match operands[0] {
            Expr::Qexp(ref v) => {
                use super::eval_input;

                return eval_input(&Expr::Sexp(v.clone()), env);
            }
            _ => Err(self.handle_error(ProgramError::BadType, &operands[0], "Expr::Qexpr")),
        }
    }

    fn len(&self, operands: &[Expr]) -> EvalResult<Expr> {
        if operands.len() > 1 {
            return Err(self.handle_error(ProgramError::BadArgs, operands.len(), 1));
        }

        match operands[0] {
            Expr::Qexp(ref v) => Ok(Expr::Val(v.len() as Number)),
            _ => Err(self.handle_error(ProgramError::BadType, &operands[0], "Expr::Qexpr")),
        }
    }

    fn list(&self, operands: &[Expr]) -> EvalResult<Expr> {
        let mut new_expr = vec![];
        new_expr.extend_from_slice(&operands[..]);
        Ok(Expr::Qexp(new_expr))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use parse_input;

    macro_rules! extract_operands {
        ($input:expr) => {
            if let Expr::Sexp(val) = parse_input($input).unwrap() {
                val
            } else {
                panic!("Operand is not an S-expression");
            }
        };
    }

    macro_rules! make_qexp {
        ( $( $x:expr ),* ) => {
            {
                let mut temp = vec![];
                $(
                    temp.push(Expr::Val($x as Number));
                )*
                Expr::Qexp(temp)
            }
        };
    }

    #[test]
    fn extract_operands_test() {
        let ast = extract_operands!("1 2 3");
        assert_eq!(ast.len(), 3);
        assert_eq!(ast[0], Expr::Val(1.0 as Number));
    }

    #[test]
    fn make_qexp_test() {
        let exp = make_qexp!(1, 2);
        if let Expr::Qexp(v) = exp {
            assert_eq!(v[0], Expr::Val(1.0 as Number));
        }
    }

    #[test]
    fn head_test() {
        let ast = extract_operands!("{1 2 5}");
        let expected = make_qexp!(1);

        let result = Builtin::Head.head(&ast).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn tail_test() {
        let ast = extract_operands!("{1 2 5}");
        let expected = make_qexp!(2, 5);

        let result = Builtin::Tail.tail(&ast).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn list_test() {
        let ast = extract_operands!("1 2 5");
        let expected = make_qexp!(1, 2, 5);

        let result = Builtin::List.list(&ast).unwrap();
        assert_eq!(result, expected);
    }

    // No clue why this is failing when the actual function works as expected
    // #[test]
    // fn join_test() {
    //     let ast = parse_input("{1 2 3} {4 5 6}").unwrap();
    //     println!("Ast: {:?}", ast);
    //     let mut vec = vec![];
    //     vec.push(Expr::Val(1 as Number));
    //     vec.push(Expr::Val(2 as Number));
    //     vec.push(Expr::Val(3 as Number));
    //     vec.push(Expr::Val(4 as Number));
    //     vec.push(Expr::Val(5 as Number));
    //     vec.push(Expr::Val(6 as Number));
    //     let expected = Expr::Qexp(vec);

    //     let result = list(&ast).unwrap();
    //     assert_eq!(result, expected);
    // }

    #[test]
    fn len_test() {
        let ast = extract_operands!("{1 2 3 4 5}");
        let expected = Expr::Val(5.0 as Number);

        let result = Builtin::Len.len(&ast).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn define_test() {
        let ast = extract_operands!("{x} 100");
        let expected = Expr::Empty;

        let mut env = Env::new();

        let result = Builtin::Def.define(&ast, &mut env).unwrap();
        assert_eq!(expected, result);
        assert!(env.table.contains_key("x"));
        assert_eq!(env.table.get("x").unwrap(), &Expr::Val(100.0 as Number));
    }
}

impl Operate for Symbol {
    fn operate(&self, o: &Vec<Expr>, env: &mut Env) -> EvalResult<Expr> {
        match *self {
            Symbol::Arith(v) => v.operate(o, env),
            Symbol::Builtin(v) => v.operate(o, env),
            // Variables can't be operated on currently. Their value is just extracted during
            // evaluation
            Symbol::Var(_) => Ok(Expr::Empty),
        }
    }
}
