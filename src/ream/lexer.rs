use std::fmt;

pub type Number = i64;

// Based on: https://www.tutorialspoint.com/lisp/lisp_quick_guide.htm

#[derive(Debug, PartialEq)]
pub enum Operations {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl fmt::Display for Operations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operations::Plus => write!(f, "+"),
            Operations::Minus => write!(f, "-"),
            Operations::Multiply => write!(f, "*"),
            Operations::Divide => write!(f, "/"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Parenthesis {
    LParen,
    RParen,
}

impl fmt::Display for Parenthesis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Parenthesis::LParen => write!(f, "("),
            Parenthesis::RParen => write!(f, ")"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenTypes {
    EOF,
    Parenthesis(Parenthesis),
    Operation(Operations),
    Number(Number), //TODO: replace with fixed point int
    Str(String),
}

impl fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenTypes::EOF => write!(f, "EOF"),
            TokenTypes::Parenthesis(c) => write!(f, "Parenthesis(' {} ')", c),
            TokenTypes::Operation(c) => write!(f, "Operation({})", c),
            TokenTypes::Number(i) => write!(f, "Number({})", i),
            TokenTypes::Str(i) => {
                let x = String::from(i);
                write!(f, "Str({})", x)
            }
        }
    }
}

pub fn lex(script: &String) -> Vec<TokenTypes> {
    // Pad parenthesis
    let mut lexed_script = script.replace("(", " ( ");
    lexed_script = lexed_script.replace(")", " ) ");

    return lexed_script
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .map(|token| {
            let parenthesis = try_parse_parenthesis(&token);
            if parenthesis.is_some() {
                return parenthesis;
            }

            let op = try_parse_op(&token);
            if op.is_some() {
                return op;
            }

            let num = try_parse_number(&token);
            if num.is_some() {
                return num;
            }

            return Some(TokenTypes::Str(token));
        })
        .filter_map(|token| token)
        .collect();
}

fn try_parse_parenthesis(token: &str) -> Option<TokenTypes> {
    return match token {
        "(" => Some(TokenTypes::Parenthesis(Parenthesis::LParen)),
        ")" => Some(TokenTypes::Parenthesis(Parenthesis::RParen)),
        _ => None,
    };
}

fn try_parse_op(token: &str) -> Option<TokenTypes> {
    return match token {
        "+" => Some(TokenTypes::Operation(Operations::Plus)),
        "-" => Some(TokenTypes::Operation(Operations::Minus)),
        "*" => Some(TokenTypes::Operation(Operations::Multiply)),
        "/" => Some(TokenTypes::Operation(Operations::Divide)),
        _ => None,
    };
}

//TODO: test
fn try_parse_number(token: &str) -> Option<TokenTypes> {
    let int = token.parse::<Number>();

    if int.is_ok() {
        return Some(TokenTypes::Number(int.unwrap()));
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_try_parse_op_not_an_op_returns_none() {
        let script = "supra test yo";
        let expected = None;
        let actual = try_parse_op(script);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lexer_try_parse_op_divide_returns_some_divide() {
        let script = "/";
        let expected = Some(TokenTypes::Operation(Operations::Divide));
        let actual = try_parse_op(script);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lexer_try_parse_op_multiply_returns_some_multiply() {
        let script = "*";
        let expected = Some(TokenTypes::Operation(Operations::Multiply));
        let actual = try_parse_op(script);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lexer_try_parse_op_plus_returns_some_plus() {
        let script = "+";
        let expected = Some(TokenTypes::Operation(Operations::Plus));
        let actual = try_parse_op(script);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lexer_try_parse_op_minus_returns_some_minus() {
        let script = "-";
        let expected = Some(TokenTypes::Operation(Operations::Minus));
        let actual = try_parse_op(script);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lexer_try_parse_parenthesis_lparen_returns_some_lparen() {
        let script = "(";
        let expected = Some(TokenTypes::Parenthesis(Parenthesis::LParen));
        let actual = try_parse_parenthesis(script);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lexer_try_parse_parenthesis_rparen_returns_some_rparen() {
        let script = ")";
        let expected = Some(TokenTypes::Parenthesis(Parenthesis::RParen));
        let actual = try_parse_parenthesis(script);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lexer_try_parse_parenthesis_not_a_parenthesis_returns_none() {
        let script = "ajajaja";
        let expected = None;
        let actual = try_parse_parenthesis(script);

        assert_eq!(expected, actual);
    }

    #[test]
    fn lexer_basic_lex() {
        let script = "(+ 1 2)".to_string();
        let tokens = lex(&script);

        let expected: Vec<TokenTypes> = vec![
            TokenTypes::Parenthesis(Parenthesis::LParen),
            TokenTypes::Operation(Operations::Plus),
            TokenTypes::Number(1),
            TokenTypes::Number(2),
            TokenTypes::Parenthesis(Parenthesis::RParen),
        ];

        assert_eq!(expected, tokens);
    }
}
