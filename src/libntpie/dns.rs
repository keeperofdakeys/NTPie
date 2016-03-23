// use libc::{addrinfo, getaddrinfo};
// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
// use std::ptr;
// use std::ffi::CString;
// 
// /// Lookup name by host
// fn lookup_name(host: &str) -> IpAddr {
//   let c_host = try!(CString::new(host));
//   let mut addr = ptr::null_mtr();
//   unsafe {
//     let res = getaddrinfo(c_host.as_ptr(), ptr::null(), ptr::null(), &mut addr);
//     match res {
//       0 => Err(()),
//       _ => 
//     }
//     if res == 0 {
//       return Err(());
//     }
//     
//   }
// }
// Copy of lookup_host crom rust libstd

use std::mem;
use std::ffi::CString;
use std::ptr;

fn sockaddr_to_addr(storage: &c::sockaddr_storage,
                    len: usize) -> io::Result<SocketAddr> {
    match storage.ss_family as c_int {
        c::AF_INET => {
            assert!(len as usize >= mem::size_of::<c::sockaddr_in>());
            Ok(SocketAddr::V4(FromInner::from_inner(unsafe {
                *(storage as *const _ as *const c::sockaddr_in)
            })))
        }
        c::AF_INET6 => {
            assert!(len as usize >= mem::size_of::<c::sockaddr_in6>());
            Ok(SocketAddr::V6(FromInner::from_inner(unsafe {
                *(storage as *const _ as *const c::sockaddr_in6)
            })))
        }
        _ => {
            Err(Error::new(ErrorKind::InvalidInput, "invalid argument"))
        }
    }
}

pub struct LookupHost {
  original: *mut c::addrinfo,
  cur: *mut c::addrinfo,
}

impl Iterator for LookupHost {
  type Item = io::Result<SocketAddr>;
  fn next(&mut self) -> Option<io::Result<SocketAddr>> {
    unsafe {
      if self.cur.is_null() { return None }
      let ret = sockaddr_to_addr(mem::transmute((*self.cur).ai_addr),
                     (*self.cur).ai_addrlen as usize);
      self.cur = (*self.cur).ai_next as *mut c::addrinfo;
      Some(ret)
    }
  }
}

unsafe impl Sync for LookupHost {}
unsafe impl Send for LookupHost {}

impl Drop for LookupHost {
  fn drop(&mut self) {
    unsafe { c::freeaddrinfo(self.original) }
  }
}

pub fn lookup_host(host: &str) -> Result<LookupHost, ()> {
  init();

  let c_host = try!(CString::new(host));
  let mut res = ptr::null_mut();
  unsafe {
    match getaddrinfo(c_host.as_ptr(), ptr::null(), ptr::null(), &mut res) {
      0 => Err(()),
      i => Ok(LookupHost { original: res, cur: res }),
    }
  }
}

