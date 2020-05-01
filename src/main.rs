use std::io::{self, Read};

pub mod ream;

fn main() {
    loop {
        println!("REPL -> ");
        let input = get_input();

        ream::repl(input);
    }
}

pub fn get_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}

enum ReplResult<TValue, TErr> {
    Ok(TValue),
    Err(TErr),
}
