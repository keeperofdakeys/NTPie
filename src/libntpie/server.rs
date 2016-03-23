use dns::lookup_host;

use std::net::{UdpSocket, IpAddr};
use std::io;
// use resolve::resolver::resolve_host;

/// A struct to hold information about an NtpServer.
pub struct NtpServer {
  /// A UDP socket used to connect to the server.
  sock: UdpSocket,
  /// The IP address of this server.
  pub addr: IpAddr,
  /// The hostname (if known) of this erver.
  pub host: String,
}

impl NtpServer {
  /// Create a new NtpServer from an ip address or hostname
  pub fn get_servers(host: &str) -> Result<Vec<Self>, NtpServerError> {
    // Get list of ips for dns name.
    let addrs = try!(
      lookup_host(host)
        .map_err(|e| NtpServerError::AddrError(e))
    );
    let servers = Vec::new();
    addrs.map(|addr| {
      // Bind udp port to ntp port of target ip.
      let socket = try!(
        UdpSocket::bind((addr, 123u16))
          .map_err(|e| NtpServerError::SockError(e))
      );
      // Generate NtpServer
      NtpServer {
        sock: socket,
        addr: addr,
        host: host.to_owned(),
      }
    }).collect::<Result<Vec<_>, NtpServerError>>()
  }
}

/// An error that occured while creating an NtpServer.
enum NtpServerError {
  /// An error from parsing a ToSocketAddrs
  AddrError(io::Error),
  /// Could not open socket
  SockError(io::Error),
}
