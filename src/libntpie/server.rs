use std::net::{UdpSocket, IpAddr, ToSocketAddr};

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
  pub fn new(host: &str) -> Result<Self, NtpServerError> {
    // Bind udp port to ntp port of target server
    let socket = UdpSocket::bind((host, 123u16));
    let socket_iter = try!(
      ToSocketAddrs((host, 123)).to_socket_addrs())
      .map_err(|e| NtpServerError::ToSocketError)
    );
    let socket = try!(socket_iter.next()
      .next().or(NtpServerError::NoSocketAddr)
    );
  }
}

/// An error that occured while creating an NtpServer.
enum NtpServerError {
  /// An error from parsing a ToSocketAddrs
  ToSocketError(std::io::Error),
  /// No socket provided
  NoSocketAddr,
  /// Could not open socket

}
