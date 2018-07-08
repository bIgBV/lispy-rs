use nom;
use nom::double_s;
use nom::IResult;

#[derive(Debug, PartialEq)]
struct Number {
    value: f64,
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Negation,
}

#[derive(Debug, PartialEq)]
enum Expr {
    Unary(Operator, Box<Expr>),
    Binary(Box<BinaryExpr>),
    Group(Box<Expr>),
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
enum Literal {
    Num(Number),
}

#[derive(Debug, PartialEq)]
struct BinaryExpr {
    op: Operator,
    lhs: Expr,
    rhs: Expr,
}

fn match_operator(op: &str) -> Operator {
    match op {
        "+" => Operator::Add,
        "-" => Operator::Subtract,
        "*" => Operator::Multiply,
        "/" => Operator::Divide,
        "%" => Operator::Modulo,
        "!" => Operator::Negation,
        _ => unreachable!("This should never happen"),
    }
}

named!(
    operator_token<&str, &str>,
    alt!(tag!("+") | tag!("-") | tag!("*") | tag!("/") | tag!("%") | tag!("!"))
);

named!(operator<&str, Operator>, map!(operator_token, |op| match_operator(op)));

named!(
    number <&str, Number>, map!(ws!(double_s), |value| Number { value })
);

named!(literal <&str, Expr>, do_parse!(
        val: number >>
        (Expr::Literal(Literal::Num(val)))
));

named!(unary_token<&str, (Operator, Expr)>,
       pair!(
           operator,
           literal
       ));

named!(unary<&str, Expr>,
       map!(unary_token, |(op, lit)| Expr::Unary(op, Box::new(lit))));

named!(binary_tokens<&str, (Expr, Operator, Expr)>,
    tuple!(
        literal,
        ws!(operator),
        literal
    ));

named!(binary<&str, Expr>,
       map!(binary_tokens, |(lhs, op, rhs)| Expr::Binary(
               Box::new(
                   BinaryExpr {
                       lhs,
                       op,
                       rhs
                   }
               )
           ))
);

fn expr(input: &str) -> IResult<&str, Expr> {
    let unary_val = unary(input);

    if unary_val.is_ok() {
        return unary_val;
    } else if binary(input).is_ok() {
        return binary(input);
    } else if literal(input).is_ok() {
        return literal(input);
    }

    Err(nom::Err::Incomplete(nom::Needed::Unknown))
}

named!(group<&str, Expr>, map!(
        delimited!(tag!("("), expr, tag!(")")),
        |res| Expr::Group(Box::new(res))
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        assert_eq!(number(&" 3.14;"[..]), Ok((";", Number { value: 3.14 })));
        assert_eq!(number(&"-3.14;"[..]), Ok((";", Number { value: -3.14 })));
        assert_eq!(number(&"8892;"[..]), Ok((";", Number { value: 8892.0 })));
    }

    #[test]
    fn test_oereator() {
        assert_eq!(operator(&"+"[..]), Ok(("", Operator::Add)));
    }

    #[test]
    fn test_literal() {
        assert_eq!(
            literal(&"18.9;"[..]),
            Ok((";", Expr::Literal(Literal::Num(Number { value: 18.9 }))))
        );
    }

    #[test]
    fn test_unary() {
        assert_eq!(
            unary(&"-9;"[..]),
            Ok((
                ";",
                Expr::Unary(
                    Operator::Subtract,
                    Box::new(Expr::Literal(Literal::Num(Number { value: 9.0 })))
                )
            ))
        );
    }

    #[test]
    fn test_binary() {
        assert_eq!(
            binary(&"7 + 9.7;"[..]),
            Ok((
                ";",
                Expr::Binary(Box::new(BinaryExpr {
                    lhs: Expr::Literal(Literal::Num(Number { value: 7.0 })),
                    op: Operator::Add,
                    rhs: Expr::Literal(Literal::Num(Number { value: 9.7 }))
                }))
            ))
        );
    }

    #[test]
    fn test_group() {
        assert_eq!(
            group(&"(7 + 9.7)"[..]),
            Ok((
                "",
                Expr::Group(Box::new(Expr::Binary(Box::new(BinaryExpr {
                    lhs: Expr::Literal(Literal::Num(Number { value: 7.0 })),
                    op: Operator::Add,
                    rhs: Expr::Literal(Literal::Num(Number { value: 9.7 })),
                }))))
            ))
        );

        assert_eq!(
            group(&"(42)"[..]),
            Ok((
                "",
                Expr::Group(Box::new(Expr::Literal(Literal::Num(Number {
                    value: 42.0
                }))))
            ))
        );

        assert_eq!(
            group(&"&(!7)"[..]),
            Ok((
                "",
                Expr::Group(Box::new(Expr::Unary(
                    Operator::Subtract,
                    Box::new(Expr::Literal(Literal::Num(Number { value: 7.0 })))
                )))
            ))
        );
    }
}
