extern crate whois_rs;

use whois_rs::server::Server;

#[test]
fn test() {
    let server = Server {
        hostname: "plop",
        port: 43
    };

    assert_eq!("plop", server.hostname);
}

#[test]
fn connect_to_iana() {
    let server = Server {
        hostname: "whois.iana.org",
        port: 43
    };

    let response = server.connect().unwrap();
    println!("ploooop");
    println!("", response);

    assert!(true);
}