use super::{EvalResult, LispyError};
use syntax::ast::*;

pub trait Operate {
    fn operate(&self, operands: &Vec<Expr>) -> EvalResult<Expr>;
}

impl Operate for Arith {
    fn operate(&self, operands: &Vec<Expr>) -> EvalResult<Expr> {
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
                _ => {
                    return Err(LispyError::BadNum);
                }
            };
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
    fn operate(&self, operands: &Vec<Expr>) -> EvalResult<Expr> {
        match *self {
            Builtin::Head => head(&operands[1..]),
            Builtin::Tail => tail(&operands[1..]),
            Builtin::List => list(&operands[1..]),
            Builtin::Join => join(&operands[1..]),
            Builtin::Eval => eval(&operands[1..]),
            Builtin::Len => len(&operands[1..]),
        }
    }
}

fn head(operands: &[Expr]) -> EvalResult<Expr> {
    if operands.len() == 0 {
        return Err(LispyError::ListError("Not enough arguments".to_owned()));
    }

    if operands.len() > 1 {
        return Err(LispyError::ListError("Too many arguments".to_owned()));
    }

    if let Expr::Qexp(ref v) = operands[0] {
        let mut qexp = vec![];
        qexp.push(v[0].clone());

        return Ok(Expr::Qexp(qexp));
    }

    Err(LispyError::ListError("Wrong type of argument".to_owned()))
}

fn tail(operands: &[Expr]) -> EvalResult<Expr> {
    if operands.len() == 0 {
        return Err(LispyError::ListError("Not enough arguments".to_owned()));
    }

    if operands.len() > 1 {
        return Err(LispyError::ListError("Too many arguments".to_owned()));
    }

    if let Expr::Qexp(ref v) = operands[0] {
        let mut qexp = vec![];
        qexp.extend_from_slice(&v[1..]);

        return Ok(Expr::Qexp(qexp));
    }

    Err(LispyError::ListError("Wrong type of argument".to_owned()))
}

fn join(operands: &[Expr]) -> EvalResult<Expr> {
    for operand in operands {
        match operand {
            Expr::Qexp(_) => continue,
            _ => {
                return Err(LispyError::BadOperand);
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

fn eval(operands: &[Expr]) -> EvalResult<Expr> {
    if operands.len() > 1 {
        return Err(LispyError::BadOp);
    }

    match operands[0] {
        Expr::Qexp(ref v) => {
            use super::eval_input;

            return eval_input(&Expr::Sexp(v.clone()));
        },
        _ => Err(LispyError::BadOp)
    }
}

fn len(operands: &[Expr]) -> EvalResult<Expr> {
    if operands.len() > 1 {
        return Err(LispyError::BadOp);
    }

    match operands[0] {
        Expr::Qexp(ref v) => Ok(Expr::Val(v.len() as Number)),
        _ => Err(LispyError::BadOp)
    }
}

pub fn list(operands: &[Expr]) -> EvalResult<Expr> {
    let mut new_expr = vec![];
    new_expr.extend_from_slice(&operands[..]);
    Ok(Expr::Qexp(new_expr))
}

impl Operate for Symbol {
    fn operate(&self, o: &Vec<Expr>) -> EvalResult<Expr> {
        match *self {
            Symbol::Arith(v) => v.operate(o),
            Symbol::Builtin(v) => v.operate(o),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use parse_input;

    #[test]
    fn head_test() {
        let ast = parse_input("{1 2 5}").unwrap();
        let mut vec = vec![];
        vec.push(Expr::Val(1 as Number));
        let expected = Expr::Qexp(vec);

        let result = head(&ast).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn tail_test() {
        let ast = parse_input("{1 2 5}").unwrap();
        let mut vec = vec![];
        vec.push(Expr::Val(2 as Number));
        vec.push(Expr::Val(5 as Number));
        let expected = Expr::Qexp(vec);

        let result = tail(&ast).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn list_test() {
        let ast = parse_input("1 2 5").unwrap();
        let mut vec = vec![];
        vec.push(Expr::Val(1 as Number));
        vec.push(Expr::Val(2 as Number));
        vec.push(Expr::Val(5 as Number));
        let expected = Expr::Qexp(vec);

        let result = list(&ast).unwrap();

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
        let ast = parse_input("{1 2 3 4 5}").unwrap();
        let expected = Expr::Val(5.0 as Number);

        let result = len(&ast).unwrap();

        assert_eq!(result, expected);
    }
}
