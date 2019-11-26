fn main() {

    let port: u16 = 29000;
    let tcp_address = format!("tcp://127.0.0.1:{}", port);
    let subscription = b"hashblock";
    
    println!("Currently listening on port {}.", port);

    let context = zmq::Context::new();

    let socket = context.socket(zmq::SUB).unwrap();
    socket.connect(&tcp_address).unwrap();
    socket.set_subscribe(subscription).unwrap();
    
    loop {
        let data = socket.recv_multipart(0).unwrap();
        println!(
            "Subscription: {}\nMessage : {}",
            std::str::from_utf8(&data[0]).unwrap(),
            hex::encode(&data[1])
        );
    }
}
