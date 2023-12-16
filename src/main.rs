mod args;
mod display;
mod lang;
mod parse;

use {
    crate::{
        args::Args,
        lang::Term,
    },
    eyre::{
        eyre,
        Result,
    },
    std::fs::read_to_string,
};

fn main() -> Result<()> {
    println!(
        "{}",
        Term::parse(&read_to_string(Args::get()?.path)?)
            .ok_or_else(|| eyre!("cannot parse the input"))?
            .evaluate_rt()
    );

    Result::Ok(())
}
