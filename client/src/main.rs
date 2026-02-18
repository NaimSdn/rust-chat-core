use std::io::{stdin, Write};
use std::net::TcpStream;

fn main() {
    let server_ip_addr: String = String::from("127.0.0.1");
    let server_port: String = String::from("8080");
    if let Ok(mut request) = TcpStream::connect(format!("{server_ip_addr}:{server_port}")) {
        println!("Connexion established with the server !");
        let mut user_input: String = String::new();
        match stdin().read_line(&mut user_input) {
            Ok(_n) => {
                let trimmed_user_input: &str = user_input.trim_matches('\n').trim_matches('\r');
                let user_byte: &[u8] = trimmed_user_input.as_bytes();
                request.write(user_byte).expect("Error on the write method !");
            }
            Err(err) => {  println!("Error : {err}"); }
        }
    } else {
        println!("Couldn't connect to the server !");
    }
}
