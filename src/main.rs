use std::net::{TcpListener};
use std::thread;
use std::io::{Read, Write};
use std::io;

fn start() -> io::Result<()> {
    let lis = try!(TcpListener::bind("0.0.0.0:12345"));
    println!("started on {:?}", lis);
    for stream in lis.incoming() {
        let mut stream = try!(stream);
        println!("{:?} connected.", try!(stream.peer_addr()));
        let _: thread::JoinHandle<io::Result<()>> = thread::spawn(move || {
            loop {
                let mut b = [0; 1024];
                let n = try!(stream.read(&mut b));
                if n == 0 {
                    println!("{:?} closed.", try!(stream.peer_addr()));
                    return Ok(());
                } else {
                    try!(stream.write(&b[0..n]));
                }
            }
        });
    }
    Ok(())
}

fn main() {
    match start() {
        Ok(_) => return,
        Err(e) =>println!("{:?}", e)
    }
}
