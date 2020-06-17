
use std::result::Result;
use std::io;
use std::io::prelude::*;
use std::str;
use std::net::TcpStream;

pub struct Server<'a> {
  pub hostname: &'a str,
  pub port: u64
}


impl Server<'_> {
  pub fn connect(&self) -> Result<String, io::Error> {
    let mut stream = TcpStream::connect(format!("{}:{}", self.hostname, self.port)).unwrap();

    let request = "com";
    stream.write(String::from(request).as_bytes()).unwrap();

    let mut response = [0; 2048];
    stream.read(&mut response).unwrap();

    let s = str::from_utf8(&response).unwrap();
    return Ok(s.to_string());
  }
}