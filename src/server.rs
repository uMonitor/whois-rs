
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
  pub fn get_domain_info(&self, part: &str) -> Result<String, io::Error> {
    if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", self.hostname, self.port)) {
      debug!("connected");

      let request = format!("{}\r\n", part);
      if let Ok(size) = stream.write(String::from(request).as_bytes()) {
        debug!("sent {} bytes", size);

        let mut response = String::from("");
        if let Ok(read_size) = stream.read_to_string(&mut response) {

          debug!("read {} bytes", read_size);
          return Ok(response.to_string());

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