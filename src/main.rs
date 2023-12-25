mod args;
mod display;
mod lang;
mod parse;

use {
    crate::{
        args::Args,
        lang::T,
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
        T::parse(&read_to_string(Args::get()?.path)?)
            .ok_or_else(|| eyre!("cannot parse the input"))?
            .eval_star()
    );

    Result::Ok(())
}
