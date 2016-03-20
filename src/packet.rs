use std::net::IpAddr;

// FIXME: Work out what this type actually is.
/// NTP Short Format
type TDist = u32;
// FIXME: Should this be a generic date value?
/// NTP Timestamp Format
type TStamp = u64;
// FIXME: Should this be a generic md5 type (if it exists)?
/// MD5 Digest
type Digest = u8;

/// An NTP Packet.
struct Packet {
  /// Source IP Address
  srcaddr: IpAddr,
  /// Destination IP Address
  dstaddr: IpAddr,
  /// Version Number
  version: u8,
  /// Leap Indicator
  leap: u8,
  /// Mode
  mode: u8,
  /// Stratum
  stratum: u8,
  /// Poll Interval
  poll: u8,
  // FIXME: This is a log(2) value, might need to be a tuplestruct.
  /// Precision
  precision: i8,
  /// Root Delay
  rootdelay: TDist,
  /// Root Dispersion
  rootdisp: TDist,
  /// Reference ID
  refid: u8,
  /// Reference Time
  reftime: TStamp,
  /// Origin Timestamp
  org: TStamp,
  /// Receive Timestamp
  rec: TStamp,
  /// Transmit Timestamp
  xmt: TStamp,
  // FIXME: These might need to be optional.
  /// Key ID
  keyid: u32,
  /// Message Digest (MD5)
  mac: Digest,
  /// Destination Timestamp
  dst: Option<TStamp>,
}
