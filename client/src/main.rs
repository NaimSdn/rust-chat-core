use std::io::{stdin, Write};
use std::net::{Shutdown, TcpStream};

fn main() {
    let server_ip_addr: String = String::from("127.0.0.1");
    let server_port: String = String::from("8080");
    let mut is_running: bool;
    if let Ok(mut stream) = TcpStream::connect(format!("{server_ip_addr}:{server_port}")) {
        let mut user_menu_choice: String = String::new();
        let mut user_input: String = String::new();
        is_running = true;
        println!("Connexion established with the server !");
        while is_running {
            println!("What do you want to do ? (Requires a single number (1 or 2)");
            println!("1 - Send a message");
            println!("2 - Quit");
            stdin().read_line(&mut user_menu_choice).expect("Error on the User menu choice ! ");
            let trimmed_user_menu_choice: &str = user_menu_choice.trim_end();
            match trimmed_user_menu_choice {
                "1" => {
                    println!("You can write your message : ");
                    match stdin().read_line(&mut user_input) {
                        Ok(_n) => {
                            let trimmed_user_input: &str = user_input.trim_end();
                            let user_byte: &[u8] = trimmed_user_input.as_bytes();
                            stream.write_all(user_byte).expect("Error on the write method !");
                        }
                        Err(err) => { println!("Error : {err}"); }
                    }
                    user_menu_choice.clear();
                    user_input.clear();
                },
                "2" => {
                    user_menu_choice.clear();
                    is_running = false;
                    stream.shutdown(Shutdown::Both).expect("Shutdown call failed !");
                },
                _ => {
                    user_menu_choice.clear();
                    println!("Not a valid choice. (Requires a single number (1 or 2) !")
                }
            }
        }
    } else {
        println!("Couldn't connect to the server !");
    }
}
