use std::net::TcpStream;

fn main() {

    /*
        TODO - CONNECT TO THE SERVER
        TODO - https://doc.rust-lang.org/std/net/struct.TcpStream.html : CONNECT
     */

    let server_ip_addr = String::from("127.0.0.1");
    let server_port = String::from("8080");

    if let Ok(_request) = TcpStream::connect(format!("{server_ip_addr}:{server_port}")) {
        println!("Connected to the server !");
    } else {
        println!("Couldn't connect to the server !");
    }

}
