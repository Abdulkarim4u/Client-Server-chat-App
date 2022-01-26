

use std::io::{ErrorKind,Read,Write}; //how to catech error , write and read from a file.
use std::net::TcpListener;
use std::sync::mpsc; //creating channels to communicate with threads
use std::thread;  //for creating threads

const LOCAL: &str = "127.0.0.1:600";
const MSG_SIZE: usize = 32;


fn main() {

    println!("Hello, world!");
}
