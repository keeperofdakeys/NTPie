use std::net::IpAddr;

// FIXME: Work out what this type actually is.
/// NTP Short Format
pub type TDist = u32;
// FIXME: Should this be a generic date value?
/// NTP Timestamp Format
pub type TStamp = u64;
// FIXME: Should this be a generic md5 type (if it exists)?
/// MD5 Digest
pub type Digest = u8;

/// An NTP Packet.
pub struct Packet {
  /// Source IP Address
  pub srcaddr: IpAddr,
  /// Destination IP Address
  pub dstaddr: IpAddr,
  /// Version Number
  pub version: u8,
  /// Leap Indicator
  pub leap: u8,
  /// Mode
  pub mode: u8,
  /// Stratum
  pub stratum: u8,
  /// Poll Interval
  pub poll: u8,
  // FIXME: This is a log(2) value, might need to be a tuplestruct.
  /// Precision
  pub precision: i8,
  /// Root Delay
  pub rootdelay: TDist,
  /// Root Dispersion
  pub rootdisp: TDist,
  /// Reference ID
  pub refid: u8,
  /// Reference Time
  pub reftime: TStamp,
  /// Origin Timestamp
  pub org: TStamp,
  /// Receive Timestamp
  pub rec: TStamp,
  /// Transmit Timestamp
  pub xmt: TStamp,
  // FIXME: These might need to be optional.
  /// Key ID
  pub keyid: u32,
  /// Message Digest (MD5)
  pub mac: Digest,
  /// Destination Timestamp
  pub dst: Option<TStamp>,
}

impl Packet {
  pub fn from_bytes(&[u8]) -> Result<Self, ()> {
    Err(())
  }
}
