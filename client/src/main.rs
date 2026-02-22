use std::io::{stdin};
use std::net::{Shutdown, TcpStream};

// TODO - Be able to send multiple messages in a row
// TODO - without the "Server" closing the connection
// TODO - nor the "Client" crashing down.

// TODO - Find a way for the "Client" to quit and
// TODO - close the connection to the "Server"
// TODO - instead of CTRL+C.

fn main() {
    let server_ip_addr: String = String::from("127.0.0.1");
    let server_port: String = String::from("8080");
    let mut is_running: bool;

    if let Ok(stream) = TcpStream::connect(format!("{server_ip_addr}:{server_port}")) {
        println!("Connexion established with the server !");

        let mut user_menu_choice: String = String::new();

        is_running = true;
        while is_running {

            println!("What do you want to do ? (Requires a single number (1 or 2)");
            println!("1 - Send a message");
            println!("2 - Quit");

            let _a = stdin().read_line(&mut user_menu_choice).expect("Error on the User menu choice ! ");
            let trimmed_user_menu_choice: &str = user_menu_choice.trim_end();

            match trimmed_user_menu_choice {
                "1" => {
                    user_menu_choice.clear();
                    println!("Choice 1 : Send a message");

                },
                "2" => {
                    user_menu_choice.clear();
                    is_running = false;

                    // TODO - Leaving the loop is correct, but i need to shutdown the TcpStream correctly.
                    // TODO - Because it still send a request to listening

                    stream.shutdown(Shutdown::Both).expect("Shutdown call failed !");

                },
                _ => {
                    user_menu_choice.clear();
                    println!("Not a valid choice. (Requires a single number (1 or 2) !")
                }
            }

            // Logic to get the Client data and
            // Send them to the Server

            // let mut user_input: String = String::new();
            // match stdin().read_line(&mut user_input) {
            //     Ok(_n) => {
            //         let trimmed_user_input: &str = user_input.trim_end();
            //         let user_byte: &[u8] = trimmed_user_input.as_bytes();
            //         request.write(user_byte).expect("Error on the write method !");
            //     }
            //     Err(err) => {  println!("Error : {err}"); }
            // }
        }
    } else {
        println!("Couldn't connect to the server !");
    }
}
