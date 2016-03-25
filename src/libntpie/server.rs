use dns_lookup::{lookup_host, LookupError};

use std::net::IpAddr;
use std::io;
// use resolve::resolver::resolve_host;

#[derive(Debug)]
/// A struct to hold information about an NtpServer.
pub struct NtpServer {
  /// The IP address of this server.
  pub addr: IpAddr,
  /// The hostname (if known) of this erver.
  pub host: String,
}

impl NtpServer {
  /// Create a new NtpServer from an ip address or hostname
  pub fn get_servers(host: &str) -> Result<Vec<Self>, NtpServerError> {
    // Get list of ips for dns name.
    let mut addrs = try!(
      try!(lookup_host(host)
        .map_err(|e| match e {
          LookupError::IOError(err) =>
            NtpServerError::AddrIOError(err),
          _ =>
            NtpServerError::AddrError
        })
      )
      .collect::<Result<Vec<_>, _>>()
      .map_err(|e| NtpServerError::AddrIOError(e))
    );
    let mut servers = Vec::new();
    addrs.sort();
    let mut last_addr = None;
    for addr in addrs {
      if let Some(a) = last_addr {
        if a == addr {
          continue;
        }
      }
      last_addr = Some(addr);
      // Generate NtpServer
      servers.push(
        NtpServer {
          addr: addr,
          host: host.to_owned(),
        }
      );
    }
    Ok(servers)
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
