use {
    clap::Parser,
    eyre::Result,
    std::path::PathBuf,
};

#[derive(Parser)]
pub struct Args {
    pub path: PathBuf,
}

impl Args {
    pub fn get() -> Result<Self> {
        Result::Ok(Self::try_parse()?)
    }
}
