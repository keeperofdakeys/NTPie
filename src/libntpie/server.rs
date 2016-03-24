use dns_lookup::lookup;

use std::net::{UdpSocket, IpAddr};
use std::io;
// use resolve::resolver::resolve_host;

#[derive(Debug)]
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
      lookup::lookup_host(host)
        .map_err(|e| match e {
          lookup::Error::IOError(err) =>
            NtpServerError::AddrIOError(err),
          _ =>
            NtpServerError::AddrError
        })
    );
    addrs.map(|addr| {
      let addr = try!(addr.map_err(|e| NtpServerError::AddrIOError(e))).ip();
      // Bind udp port to ntp port of target ip.
      let socket = try!(
        UdpSocket::bind(("127.0.0.1", 0u16))
          .map_err(|e| NtpServerError::SockError(e))
      );
      // Generate NtpServer
      Ok(NtpServer {
        sock: socket,
        addr: addr,
        host: host.to_owned(),
      })
    }).collect::<Result<Vec<_>, NtpServerError>>()
  }
}

#[derive(Debug)]
/// An error that occured while creating an NtpServer.
pub enum NtpServerError {
  /// Error looking up address
  AddrError,
  /// Error looking up address (with attached IO Error)
  AddrIOError(io::Error),
  /// Could not open socket
  SockError(io::Error),
}
