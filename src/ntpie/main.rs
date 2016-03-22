use std::net::ToSocketAddrs;

use libntpie::packet::TStamp;
use libntpie::server::NtpServer;

fn main() {
  let server = NtpServer::new();
}

fn get_time<A: ToSocketAddrs>(addr: A) -> TStamp {
  
}
