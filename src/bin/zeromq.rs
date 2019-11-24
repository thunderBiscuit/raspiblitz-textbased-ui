fn main() {
    println!("Hello, world!");

    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::SUB).unwrap();
    // let socket = ctx.socket(zmq::PULL).unwrap();
    // let socket = ctx.socket(zmq::STREAM).unwrap();
    socket.connect("tcp://127.0.0.1:28332").unwrap();
    loop {
        let data = socket.recv_multipart(0).unwrap();
        println!(
            "Identity: {:?} Message : {}",
            data[0],
            hex::encode(&data[1])
        );
    }
}
