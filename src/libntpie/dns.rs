use libc::{addrinfo, getaddrinfo};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::ptr;
use std::ffi::CString;

/// Lookup name by host
fn lookup_name(host: &str) -> IpAddr {
  let c_host = try!(CString::new(host));
  let mut addr = ptr::null_mtr();
  unsafe {
    let res = getaddrinfo(c_host.as_ptr(), ptr::null(), ptr::null(), &mut addr);
    match res {
      0 => Err(()),
      _ => *
    }
    if res == 0 {
      return Err(());
    }
    
  }
}
