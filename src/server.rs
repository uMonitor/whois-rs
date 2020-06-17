
extern crate log;

use log::{debug, error};
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
    if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", self.hostname, self.port)) {
      debug!("connected");

      let request = "com";
      if let Ok(size) = stream.write(String::from(request).as_bytes()) {
        debug!("sent {} bytes", size);

        let mut response = [0; 2048];
        if let Ok(read_size) = stream.read(&mut response) {

          let s = str::from_utf8(&response).unwrap();
          debug!("read {} bytes", read_size);
          return Ok(s.to_string());

        }
      }
      else {
        error!("sent failed");
      }
    }
    else {
      error!("Not connected");
    }

    return Ok(String::from("crap"));
  }
}