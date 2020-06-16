extern crate whois_rs;

use whois_rs::split::split;

#[test]
fn test1() {
  let v = split("umonitor.io");

  assert_eq!("io", v[1]);
}