use std::fmt;

pub type Number = i64;

#[derive(Debug, PartialEq)]
pub enum TokenTypes {
    EOF,
    Parenthesis(char),
    Operation(String),
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

fn try_parse_parenthesis(token: &str) -> Option<TokenTypes> {
    return match token {
        "(" => Some(TokenTypes::Parenthesis('(')),
        ")" => Some(TokenTypes::Parenthesis(')')),
        _ => None,
    };
}

fn try_parse_op(token: &str) -> Option<TokenTypes> {
    return match token {
        "+" => Some(TokenTypes::Operation(token.to_string())),
        _ => None,
    };
}

fn try_parse_number(token: &str) -> Option<TokenTypes> {
    let int = token.parse::<Number>();

    if int.is_ok() {
        return Some(TokenTypes::Number(int.unwrap()));
    }

    return None;
}

pub fn lex(script: &String) -> Vec<TokenTypes> {
    // Pad parenthesis
    let mut lexed_script = script.replace("(", " ( ");
    lexed_script = lexed_script.replace(")", " ) ");

    return lexed_script
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_basic_lex() {
        let script = "(+ 1 2)".to_string();
        let tokens = lex(&script);

        let expected: Vec<TokenTypes> = vec![
            TokenTypes::Parenthesis('('),
            TokenTypes::Operation("+".to_string()),
            TokenTypes::Number(1),
            TokenTypes::Number(2),
            TokenTypes::Parenthesis(')'),
        ];

        assert_eq!(expected, tokens);
    }
}
