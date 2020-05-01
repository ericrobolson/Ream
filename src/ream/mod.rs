mod lexer;
use lexer::lex;

pub fn repl(script: String) {
    let tokens = read(&script);
}

fn read(script: &String) {
    let tokens = lex(script);

    for token in tokens {
        println!("read: {}", token);
    }
}

fn evaluate() {}

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
