mod display;
mod lang;
mod parse;

use {
    crate::lang::Term,
    std::io::{
        stdin,
        stdout,
        Result,
        Write as _,
    },
};

fn main() -> Result<()> {
    let stdin = stdin();
    let mut stdout = stdout();
    let mut input = String::new();

    loop {
        print!("> ");
        stdout.flush()?;
        stdin.read_line(&mut input)?;

        match Term::parse(&input) {
            Option::Some(term) => println!("{}", term.evaluate_rt()),
            Option::None => println!("cannot parse the input"),
        }

        input.clear();
    }
}
