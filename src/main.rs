use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::thread;

fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:12345";
    let client_socket = UdpSocket::bind("0.0.0.0:0")?;
    let server_addr: SocketAddr = server_address.parse().expect("Invalid server address");
    client_socket
        .connect(server_addr)
        .expect("Can't connect with the server");

    let mut buffer = [0; 1024];
    client_socket.send("ping".as_bytes())?;

    println!("Welcome to Hangman!");
    let receive_updates_socket = client_socket.try_clone()?;
    let server_addr_clone = server_addr;
    let receive_thread = thread::spawn(move || loop {
        match receive_updates_socket.recv_from(&mut buffer) {
            Ok((received, _server_address)) => {
                let response = String::from_utf8_lossy(&buffer[..received]);
                println!("{}", response);
                if response.contains("Game over!") {
                    break;
                }
            }
            Err(err) => {
                eprintln!("Error in thread: {}", err);
                break;
            }
        }
    });
    loop {
        let send_thread = thread::spawn(move || {
            let mut user_input = String::new();
            println!("Enter Your Guess: ");
            if io::stdin().read_line(&mut user_input).is_err() {
                eprintln!("Failed to read line");
            }
            user_input
        });
        let input = send_thread.join().unwrap();

        if let Err(err) = client_socket.send_to(input.as_bytes(), server_addr) {
            eprintln!("Error sending data to the server: {}", err);
            break;
        }
    }

    Ok(())
}
