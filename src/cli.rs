use clap::Parser;
use url::Url;

#[derive(Debug, Parser)]
#[clap(name = "snag", version = "0.1.0", author = "Gsmith")]
pub struct Cli {
    /// The url to fetch the data from
    #[arg(long, short = 'u')]
    url: String,
}

impl Cli {
    pub fn run(&self) -> Result<(), String> {
        Url::parse(&self.url).map_err(|err| format!("Invalid url '{}': {}", &self.url, err))?;

        Ok(())
    }
}
