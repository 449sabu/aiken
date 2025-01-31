use chumsky::prelude::*;

use crate::{
    ast,
    expr::UntypedExpr,
    parser::{error::ParseError, token::Token},
};

use super::block;

pub fn parser<'a>(
    sequence: Recursive<'a, Token, UntypedExpr, ParseError>,
    expression: Recursive<'a, Token, UntypedExpr, ParseError>,
) -> impl Parser<Token, UntypedExpr, Error = ParseError> + 'a {
    just(Token::If)
        .ignore_then(
            expression
                .clone()
                .then(block(sequence.clone()))
                .map_with_span(|(condition, body), span| ast::IfBranch {
                    condition,
                    body,
                    location: span,
                }),
        )
        .then(
            just(Token::Else)
                .ignore_then(just(Token::If))
                .ignore_then(
                    expression
                        .clone()
                        .then(block(sequence.clone()))
                        .map_with_span(|(condition, body), span| ast::IfBranch {
                            condition,
                            body,
                            location: span,
                        }),
                )
                .repeated(),
        )
        .then_ignore(just(Token::Else))
        .then(block(sequence))
        .map_with_span(|((first, alternative_branches), final_else), span| {
            let mut branches = vec1::vec1![first];

            branches.extend(alternative_branches);

            UntypedExpr::If {
                location: span,
                branches,
                final_else: Box::new(final_else),
            }
        })
}

#[cfg(test)]
mod tests {
    use crate::assert_expr;

    #[test]
    fn if_else_basic() {
        assert_expr!(
            r#"
            if True {
              1 + 1
            } else if a < 1 {
              3
            } else {
              4
            }
            "#
        );
    }
}
