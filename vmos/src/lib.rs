use socket2::{Domain, SockAddr, Socket, Type};
use std::error::Error;
use std::net::{SocketAddr, TcpListener};


#[allow(dead_code)]
fn server() -> Result<(), Box<dyn Error>> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    let addr: SockAddr = "0.0.0.0:9999".parse::<SocketAddr>()?.into();
    socket.bind(&addr).expect("bind");
    socket.listen(128).expect("listener");
    let listener: TcpListener = socket.into();
    loop {
        let (_sock, addr) = listener.accept().expect("accept");
        println!("peer addr: {}", addr)
    }
}
