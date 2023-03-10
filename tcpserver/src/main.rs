use std::io::{Read, Write};
// use std::io::Write;
use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("running on port 3000...");
    // 如果仅仅只接收一次连接
    // let res = listener.accept().unwrap();
    // 多次接收连接，遍历下面字节流
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
