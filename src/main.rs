

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

            println!("client{}connected ",addr);
            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));

            thread::spawn(move || loop {

                let mut buff = vec![0;MSG_SIZE];
                match socket.read_exact(&mut buff){
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x|x!=0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("invalid utf8 message");
                        println!("{}: {:?}",addr,msg);
                        tx.send(msg).expect("failed to send the message rx");
                    },
                    Err(ref err)if err.kind() == ErrorKind::WouldBlock =>(),
                    Err(_)=>{
                        println!("Closing connection with {}",addr);
                        break;
                    }



                }
            })
        }
    }



    println!("Hello, world!");
}
