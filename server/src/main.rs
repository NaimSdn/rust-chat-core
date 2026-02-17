use std::io::Read;
use std::net::TcpListener;

fn main() {
    let server_ip_addr: String = String::from("127.0.0.1");
    let server_port: String = String::from("8080");
    let listener: TcpListener = TcpListener::bind(format!("{server_ip_addr}:{server_port}")).unwrap();
    for request in listener.incoming() {
        match request {
            Ok( mut n) => {
                println!("Connexion established with the client !");
                let mut buffer: [u8; 50] = [0; 50];
                let request_bytes_size = n.read(&mut buffer).expect("Error on the read method !");
                let words: String = String::from_utf8(buffer[..request_bytes_size].to_vec()).unwrap();
                println!("Client words : {words}");
            }
            Err(err) => { println!("Connexion failed error : {err}") }
        }
    }
}
