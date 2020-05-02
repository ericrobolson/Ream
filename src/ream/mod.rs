mod lexer;
use lexer::{lex, TokenTypes};

use fixed::types::I20F12;
pub type Number = I20F12;

pub fn repl(script: String) {
    let tokens = lex(&script);
    let res = evaluate(&tokens);
}

fn evaluate(tokens: &Vec<lexer::TokenTypes>) {
    let mut opened_paren = 0;

    let mut op_stack = vec![];
    let mut arg_stack = vec![];

    for (i, token) in tokens.iter().enumerate() {
        match token {
            TokenTypes::Parenthesis(p) => match p {
                lexer::Parenthesis::LParen => {
                    opened_paren += 1;
                }
                lexer::Parenthesis::RParen => {
                    opened_paren -= 1;
                }
            },
            TokenTypes::Operation(op) => {
                op_stack.push(op);
            }
            TokenTypes::Number(v) => {
                arg_stack.push(v);
            }
            _ => {}
        }
    }

    if opened_paren != 0 {
        panic!("Error parsing parenthesis!");
    }

    while op_stack.is_empty() == false {
        let op = op_stack.pop();

        if op.is_some() {
            let op = op.unwrap();

            let mut value = Number::from_num(0);
            match op {
                lexer::Operations::Plus => {
                    for arg in &arg_stack {
                        value += *arg;
                    }
                }
                lexer::Operations::Minus => {
                    for arg in &arg_stack {
                        value -= *arg;
                    }
                }
                lexer::Operations::Multiply => {
                    for (arg_idx, arg) in arg_stack.iter().enumerate() {
                        if arg_idx == 0 {
                            value = **arg;
                        } else {
                            value = value * (*arg);
                        }
                    }
                }
                lexer::Operations::Divide => {
                    for (arg_idx, arg) in arg_stack.iter().enumerate() {
                        if **arg == Number::from_num(0) {
                            panic!("Divide by zero!");
                        }

                        if arg_idx == 0 {
                            value = **arg;
                        } else {
                            value = value / (*arg);
                        }
                    }
                }
            }

            println!("-> {}", value.to_string());
        }
    }
}

enum ReamObjects {
    Atom,
    List,
    Str(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_test() {
        let script = "(+ 1 1)".to_string();

        let result = repl(script);
        unimplemented!();
        //assert_eq!(2, result);
    }
}
