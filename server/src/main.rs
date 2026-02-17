use std::net::TcpListener;

fn main() {
    /*
        TODO - START A SERVER
        TODO - https://doc.rust-lang.org/std/net/struct.TcpListener.html : BIND / INCOMING METHODS
     */

    let server_ip_addr = String::from("127.0.0.1");
    let server_port = String::from("8080");

    let listener = TcpListener::bind(format!("{server_ip_addr}:{server_port}")).unwrap();

    for request in listener.incoming() {
        match request {
            Ok(_request) => {
                println!("Connexion established !");
            }
            Err(err) => { println!("Connexion failed error : {err}")}
        }
    }
}
