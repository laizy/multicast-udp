///! 源主机通过往主播地址(224.0.0.0-239.255.255.255的D类地址)发送报文，如果有其他主机对于这个组的报文有兴趣
/// 可以申请加入这个组，就可以收到源主机发出的报文,而其他不是这个组的成员无法接受到这个组的报文。
use std::net::{UdpSocket ,SocketAddrV4 ,Ipv4Addr};
use std::thread;
use std::env;
use std::time::Duration;

fn server() {
    let socket = UdpSocket::bind("0.0.0.0:8888").unwrap();
    let mut buf = [0u8;65535];
    let multi_addr = Ipv4Addr::new(234,2,2,2);
    let local = Ipv4Addr::new(0,0,0,0);
    socket.join_multicast_v4(&multi_addr, &local).unwrap();

    loop {
        let (amount, src) = socket.recv_from(&mut buf).unwrap();
        println!("received {} bytes from {:?}", amount, src);
    }
}

fn client() {
    let port = rand::random::<u16>() % 100 + 9000;
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    let socket = UdpSocket::bind(addr).unwrap();
    let buf = [1u8; 1500];
    let count = 1473;

    for _i in 0..100 {
        socket.send_to(&buf[0..count], "234.2.2.2:8888").unwrap();
        thread::sleep(Duration::from_secs(1));
    }

}

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} server/client", args[0]);
        return
    }
    if args[1] == "server" {
       server()
    } else if args[1] == "client" {
        client()
    } else {
        println!("usage: {} server/client", args[0]);
    }
}

