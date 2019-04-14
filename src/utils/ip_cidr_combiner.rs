use crate::cidr::{IpCidr, Ipv4Cidr, Ipv6Cidr};
use crate::utils::{Ipv4CidrCombiner, Ipv6CidrCombiner};
use std::fmt::{self, Formatter, Debug, Display};
use core::fmt::Write;
use std::net::IpAddr;

/// To combine multiple IPv4 CIDRs and IPv6 CIDRs to supernetworks.
pub struct IpCidrCombiner {
    ipv4: Ipv4CidrCombiner,
    ipv6: Ipv6CidrCombiner,
}

impl Debug for IpCidrCombiner {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if f.alternate() {
            f.write_fmt(format_args!("IpCidrCombiner {{\n    ipv4: {:#?},\n    ipv6: {:#?}\n}}", self.ipv4, self.ipv6))
        } else {
            f.write_fmt(format_args!("{{ ipv4: {:?}, ipv6: {:?} }}", self.ipv4, self.ipv6))
        }
    }
}

impl Display for IpCidrCombiner {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_char('[')?;

        let ipv4_length = self.ipv4.len();

        if ipv4_length > 0 {
            let length_dec = ipv4_length - 1;

            for cidr in self.ipv4.iter().take(length_dec) {
                f.write_fmt(format_args!("{}, ", cidr))?
            }

            f.write_fmt(format_args!("{}", self.ipv4[length_dec]))?;
        }

        let ipv6_length = self.ipv6.len();

        if ipv6_length > 0 {
            let length_dec = ipv6_length - 1;

            if ipv4_length > 0 {
                f.write_str(", ")?;
            }

            for cidr in self.ipv6.iter().take(length_dec) {
                f.write_fmt(format_args!("{}, ", cidr))?
            }

            f.write_fmt(format_args!("{}", self.ipv6[length_dec]))?;
        }

        f.write_char(']')
    }
}

impl IpCidrCombiner {
    #[inline]
    /// Create a new `IpCidrCombiner` instance.
    pub fn new() -> IpCidrCombiner {
        IpCidrCombiner {
            ipv4: Ipv4CidrCombiner::new(),
            ipv6: Ipv6CidrCombiner::new(),
        }
    }

    #[inline]
    /// Create a new `IpCidrCombiner` instance with specific capacities.
    pub fn with_capacity(ipv4_capacity: usize, ipv6_capacity: usize) -> IpCidrCombiner {
        IpCidrCombiner {
            ipv4: Ipv4CidrCombiner::with_capacity(ipv4_capacity),
            ipv6: Ipv6CidrCombiner::with_capacity(ipv6_capacity),
        }
    }
}

impl IpCidrCombiner {
    #[inline]
    pub fn get_ipv4_cidrs(&self) -> &[Ipv4Cidr] {
        &self.ipv4
    }

    #[inline]
    pub fn get_ipv6_cidrs(&self) -> &[Ipv6Cidr] {
        &self.ipv6
    }
}

impl IpCidrCombiner {
    /// Push a CIDR into this combiner.
    pub fn push(&mut self, cidr: IpCidr) {
        match cidr {
            IpCidr::V4(cidr) => {
                self.ipv4.push(cidr);
            }
            IpCidr::V6(cidr) => {
                self.ipv6.push(cidr);
            }
        }
    }

    #[inline]
    /// Check an IP whether it is in these CIDRs.
    pub fn contains(&self, ip: IpAddr) -> bool {
        match ip {
            IpAddr::V4(ipv4) => {
                for cidr in self.ipv4.iter() {
                    if cidr.contains(&ipv4) {
                        return true;
                    }
                }
            }
            IpAddr::V6(ipv6) => {
                for cidr in self.ipv6.iter() {
                    if cidr.contains(&ipv6) {
                        return true;
                    }
                }
            }
        }

        false
    }
}