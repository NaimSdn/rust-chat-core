use std::io::Read;
use std::net::TcpListener;
use std::thread;

fn main() {
    let server_ip_addr: String = String::from("127.0.0.1");
    let server_port: String = String::from("8080");
    let listener: TcpListener = TcpListener::bind(format!("{server_ip_addr}:{server_port}")).unwrap();
    for listener_incoming in listener.incoming() {
        match listener_incoming {
            Ok( mut n) => {
                let _thread_stream = thread::spawn(move || {
                    println!("Connexion established with the client !");
                    loop {
                        let mut buffer: [u8; 50] = [0; 50];
                        let request_bytes_size = n.read(&mut buffer).expect("Error on the read method !");
                        if request_bytes_size == 0 {
                            break;
                        }
                        let words: String = String::from_utf8(buffer[..request_bytes_size].to_vec()).unwrap();
                        println!("Client words : {words}");
                    }
                });
            }
            Err(err) => { println!("Connexion failed error : {err}") }
        }
    }
}
