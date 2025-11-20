fn print_help() {
    println!("Usage: streamchat

Stream cipher chat with Diffie-Hellman key generation

Commands:
    server   Start server
    client   Connect to server
");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 || args.contains(&String::from("--help")) {
        print_help();
return;
}

match args[1].as_str() {
        "server" => run_server(),
        "client" => run_client(),
        _ => print_help(),       
}
}
fn run_server() {
    use std::net::{TcpListener, TcpStream};
    use std::io::{Read, Write};
    use std::thread;

    const LOCAL: &str = "127.0.0.1:6000";
    const MSG_SIZE: usize = 128;

    let server = TcpListener::bind(LOCAL).expect("Failed to bind");
    println!("Server listening on {}", LOCAL);

    for stream in server.incoming() {
        match stream {
            Ok(mut socket) => {
                println!("Client connected: {}", socket.peer_addr().unwrap());
                thread::spawn(move || {
                    let mut buf = [0u8; MSG_SIZE];
                    loop {
                        match socket.read(&mut buf) {
                            Ok(0) => break,
                            Ok(n) => {
                                let msg = String::from_utf8_lossy(&buf[..n]);
                                print!("<client>: {}", msg);
                                // Echo back
                                socket.write_all(msg.as_bytes()).unwrap();
                            }
                            Err(_) => break
                        }
                    }
                    println!("Client disconnected");
                });
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}

fn run_client() {
    use std::net::TcpStream;
    use std::io::{self, Write, Read};

    const LOCAL: &str = "127.0.0.1:6000";
    const MSG_SIZE: usize = 128;

    let mut stream = TcpStream::connect(LOCAL).expect("Failed to connect");
    println!("Connected to {}", LOCAL);

    let mut input = String::new();
    loop {
        input.clear();
        print!("You: ");
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            break;
        }
        stream.write_all(input.as_bytes()).unwrap();

        let mut buf = [0u8; MSG_SIZE];
        match stream.read(&mut buf) {
            Ok(n) if n == 0 => break,
            Ok(n) => println!("<server echo>: {}", String::from_utf8_lossy(&buf[..n])),
            Err(_) => break,
        }
    }
    println!("Disconnected");
}

