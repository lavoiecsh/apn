use apn::Environment;
use std::io::Write;
use std::{env, io};

macro_rules! print_stack {
    ($environment: tt) => {
        $environment
            .stack()
            .enumerate()
            .for_each(|(i, e)| println!("{:3}: {:?}", i, e));
    };
}

fn main() -> Result<(), io::Error> {
    let mut environment = Environment::new();
    let mut input = env::args()
        .skip(1)
        .fold(String::new(), |acc, cur| acc + " " + &cur);
    if !input.is_empty() {
        return if let Err(e) = environment.evaluate(&input) {
            println!("{:?}", e);
            Err(io::Error::new(io::ErrorKind::InvalidInput, e))
        } else {
            print_stack!(environment);
            Ok(())
        };
    }

    loop {
        print!("> ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();
        if input == "exit" {
            break;
        }
        if let Err(e) = environment.evaluate(&input) {
            println!("Error evaluating: {:?}", e);
        }
        print_stack!(environment);
    }
    Ok(())
}
