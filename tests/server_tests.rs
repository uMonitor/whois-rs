extern crate whois_rs;

#[cfg(test)]
mod tests {
    use whois_rs::server::Server;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test() {
        init();

        let server = Server {
            hostname: "plop",
            port: 43,
        };

        assert_eq!("plop", server.hostname);
    }

    #[test]
    fn connect_to_iana() {
        init();
        let server = Server {
            hostname: "whois.iana.org",
            port: 43,
        };

        let response = server.connect("com").unwrap();
        println!("{}", response);

        assert!(!response.is_empty());
    }
}
