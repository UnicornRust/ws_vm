use socket2::{Domain, SockAddr, Socket, Type};
use std::error::Error;
use std::net::{SocketAddr, TcpListener};
use win;

fn main() {
    println!("Hello, world!");
    let label = win::add(2, 3);
    println!("{}", label);
}

fn server() -> Result<(), Box<dyn Error>> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    let addr: SockAddr = "0.0.0.0:9999".parse::<SocketAddr>()?.into();
    socket.bind(&addr).expert("bind");
    socket.listener(128).expect("listener");
    let listener: TcpListener = socket.into();
    loop {
        let (_sock, addr) = listener.accept().expect("accept");
        println!("peer addr: {}", addr)
    }
}
