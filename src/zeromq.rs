use std::str;

fn main() {
    println!("Hello, RaspiBlitz!");

    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::STREAM).unwrap();
    socket.bind("tcp://127.0.0.1:28332").unwrap();
    loop {
        let data = socket.recv_multipart(0).unwrap();
        println!(
            "Identity: {:?} Message : {}",
            data[0],
            str::from_utf8(&data[1]).unwrap()
        );
    }
}