use clap::Parser;
use url::Url;

#[derive(Debug, Parser)]
#[clap(name = "snag", version = "0.1.0", author = "Gsmith")]
pub struct Cli {
    /// The url to fetch the data from
    #[arg(long, short)]
    url: String,

    /// Http verb. Defaults to `GET` if not provided
    #[arg(long, short, default_value_t = String::from("GET"))]
    method: String,
}

impl Cli {
    pub fn run(&self) -> Result<(), String> {
        let url =
            Url::parse(&self.url).map_err(|err| format!("Invalid url '{}': {}", &self.url, err))?;

        let host = url.host_str().unwrap_or_else(|| "No host found in the url");

        println!("connecting to {}", host);
        println!(
            "Sending request {} {} {}",
            &self.method,
            url.path(),
            "HTTP/1.1"
        );
        println!("Host: {}", host);
        println!("Accept: */*");

        Ok(())
    }
}
