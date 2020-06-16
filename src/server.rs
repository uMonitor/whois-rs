
use std::result::Result;
use std::io;
use std::net::TcpStream;

pub struct Server<'a> {
  pub hostname: &'a str,
  pub port: u64
}


impl Server<'_> {
  fn connect() -> Result<String, io::Error> {
    let mut stream = TcpStream::connect("whois.iana.org:43")?;

    stream.write("com")?;
    stream.read(&mut [0; 128])?;

    Ok();
  }
}