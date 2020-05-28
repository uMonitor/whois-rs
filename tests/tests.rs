extern crate whois_rs;

use whois_rs::Server;

#[test]
fn test() {
    let who = Server {
        hostname: "plop",
        port: 43
    };

    assert_eq!("plop", who.hostname);

}