pub fn lex(script: &String) -> Vec<String> {
    // Pad parenthesis
    let mut lexed_script = script.replace("(", " ( ");
    lexed_script = lexed_script.replace(")", " ) ");

    let mut tokens = vec![];
    for token in lexed_script.split_whitespace().map(|s| s.to_string()) {
        tokens.push(token);
    }

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_basic_lex() {
        let script = "(+ 1 2)".to_string();
        let tokens = lex(&script);

        let expected = vec!["(", "+", "1", "2", ")"];

        assert_eq!(expected, tokens);
    }
}
