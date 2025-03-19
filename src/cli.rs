use std::{
    io::{Read, Write},
    net::TcpStream,
};

use clap::{Parser, ValueEnum};
use native_tls::TlsConnector;
use url::Url;

#[derive(Debug, Clone, ValueEnum)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::DELETE => write!(f, "DELETE"),
        }
    }
}

#[derive(Debug, Parser)]
#[clap(name = "snag", version = "0.1.0", author = "Gsmith")]
pub struct Cli {
    /// The url to fetch the data from
    #[arg(long, short)]
    url: String,

    /// Http verb. Defaults to `GET` if not provided
    #[arg(long, short, value_enum, default_value_t = HttpMethod::GET)]
    method: HttpMethod,
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

        let result = self
            .send_request(url)
            .map_err(|err| format!("Error sending request: {}", err));

        println!("Result after successful call: {:#?}", result);

        Ok(())
    }

    fn send_request(&self, url: Url) -> Result<String, Box<dyn std::error::Error>> {
        let host = url.host_str().ok_or("No host found in the url")?;
        let path = url.path();
        let port = url
            .port()
            .unwrap_or(if url.scheme() == "https" { 443 } else { 80 });

        let connector = TlsConnector::new()?;

        let stream = TcpStream::connect(format!("{}:{}", host, port))?;
        let mut stream = connector.connect(host, stream)?;

        let request = format!(
            "{} {} HTTP/1.1\r\n\
             Host: {}\r\n\
             Connection: close\r\n\
             \r\n",
            self.method, path, host,
        );

        println!("Sending request: \n{}", request);

        stream.write_all(request.as_bytes())?;

        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        println!("Response:: {}", response);
        Ok(response)
    }
}
