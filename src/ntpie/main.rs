extern crate libntpie;

use std::net::UdpSocket;

use libntpie::server::NtpServer;

fn main() {
  // Bind udp port to ntp port of target ip.
  let socket = UdpSocket::bind(("127.0.0.1", 0u16)).unwrap();
  let servers = NtpServer::get_servers("0.gentoo.pool.ntp.org").unwrap();
  println!("{:?}", servers);
}

// fn get_time<A: ToSocketAddrs>(addr: A) -> TStamp {
//   
// }
