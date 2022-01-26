

use std::io::{ErrorKind,Read,Write}; //how to catch error , write and read from a file.
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc; //creating channels to communicate with threads
use std::thread;
use proc_macro::bridge::PanicMessage::String;  //for creating threads

const LOCAL: &str = "127.0.0.1:600";
const MSG_SIZE: usize = 32;


fn main() {

    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("failed to initialize non blocking");
    let mut clients = vec![];
    let (tx,rx) = mpsc::channel::<String>();

    loop {
        if let Ok((mut Socket , addr ))=server.accept(){

            println!("client connected {}",addr);
        }
    }



    println!("Hello, world!");
}
