use calculator::Error;

use crate::calculator::Calculator;

mod calculator;

fn main() -> Result<(), Error> {
    loop {
        let mut input = String::new();

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let tokens = Calculator::parse(input);

                if tokens.is_err() {
                    println!("Error: {:?}", tokens.err().unwrap());
                    continue;
                }

                let expression = Calculator::expression(tokens?);

                if let Some(value) = Calculator::evaluate(expression) {
                    println!("{}", value);
                }
            }
            Err(error) => println!("Error: {:?}", error)
        }

    }
}
