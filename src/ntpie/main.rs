extern crate libntpie;

use std::net::ToSocketAddrs;

use libntpie::server::NtpServer;

fn main() {
  let servers = NtpServer::get_servers("0.gentoo.pool.ntp.org").unwrap();
  println!("{:?}", servers);
}

// fn get_time<A: ToSocketAddrs>(addr: A) -> TStamp {
//   
// }
