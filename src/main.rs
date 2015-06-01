use std::net::{TcpListener};
use std::thread;
use std::io::{Read, Write};

fn main() {
    if let Ok(lis) = TcpListener::bind("0.0.0.0:12345") {
        println!("started on {:?}", lis);
        for stream in lis.incoming() {
            if let Ok(mut stream) = stream {
                println!("{:?} connected.", stream.peer_addr());
                thread::spawn(move || {
                    loop {
                        let mut b = [0; 1024];
                        if let Ok(n) = stream.read(&mut b) {
                            if n == 0 {
                                println!("{:?} closed.", stream.peer_addr());
                                return;
                            } else {
                                let _ = stream.write(&b[0..n]);
                            }
                        } else {
                            return;
                        }
                    }
                });
            }
        }
    }
}

