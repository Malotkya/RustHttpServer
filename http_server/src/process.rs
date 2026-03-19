use async_lib::{
    io::{Result, ErrorKind},
    net::{TcpListener, TcpStream},
    executor::{
        is_running, shut_down,
        thread::ThreadProcess
    }
};

pub fn read_stdin() {
    let stdin = std::io::stdin();
    let mut input = String::new();

    println!("Enter \"quit\" to shutdown server!");
    while is_running() {
        stdin.read_line(&mut input).unwrap();

        match input.to_lowercase().trim() {
            "quit" => {
                shut_down();
                break;
            },
            _ => println!("Unknown command \"{}\"", input)
        }
    }
}

pub fn tcp_listener(addr:String, callback:impl Fn(TcpStream) + Send + Sync + 'static) -> Result<impl ThreadProcess> {
    let mut listener = TcpListener::bind(addr.clone())?;

    if listener.set_nonblocking(true).is_err() {
        println!("Failed to set nonblocking on TcpListener!");
    }

    Ok(move ||{
        println!("Listening at {}", addr);

        while is_running() {
            match listener.sync_accept() {
                Ok(conn) => {
                    callback(conn.0);
                },
                Err(e) => if e.kind() != ErrorKind::WouldBlock {
                    println!("Connection Error: {}", e);
                } 
            }
        }
    })
}