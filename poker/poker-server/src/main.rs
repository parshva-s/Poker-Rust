use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};

#[cfg(test)]
pub mod dealer;

fn game_session_selection(stream: &mut TcpStream)
{
    // TODO: implement game session selection by spawning all games that are available with current players in the game
    // TODO: send a list of games to the client

    // TODO: once session is selected, start the game which should be a dealer game
}

fn verify_user(stream: &mut TcpStream) {
    let mut buf = [0; 512];
    let mut is_new_user = false;
    let username_entered = false;
    let mut username = String::new();
    loop {
        let bytes_read = stream.read(&mut buf).unwrap();
        // split the buffer into a vector of strings separated by spaces
        let tokens: Vec<&str> = std::str::from_utf8(&buf[..bytes_read]).unwrap().split_whitespace().collect();
        if tokens.len() == 0 {
            continue
        } else if tokens[0] == "u" {
            // username
            username = tokens[1].to_owned();
            let mut response = "";
            // if username is in database, send a response to ask for password
            // response = "returning user";
            // else, send a response to ask for password
            response = "new user";
            stream.write(response.as_bytes()).unwrap();
        } else if tokens[0] == "p" && username_entered {
            let password = tokens[1];
            if is_new_user {
                // TODO: create new user with password saved
            } else {
                // TODO: verify password
            }
            let response: &str = "pass good";
            stream.write(response.as_bytes()).unwrap();
            game_session_selection(stream);
        } else if tokens[0] == "q" {
            // quit
            let response = "Goodbye!";
            stream.write(response.as_bytes()).unwrap();
            break;
        }
    }
}

fn setup_server() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).unwrap();
    println!("Server listening on {}", addr);
    for stream in listener.incoming() {
        match stream {
            // create multiple threads to handle multiple clients
            Ok(mut stream) => {
                let thread = std::thread::spawn(move || {
                    verify_user(&mut stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn setup_database() {

}

fn main() {
    setup_database();
    setup_server();
}
