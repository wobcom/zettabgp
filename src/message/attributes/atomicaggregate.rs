// Copyright 2021 Vladimir Melnikov.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! BGP "Atomic aggregate" path attribute

use crate::*;
use crate::message::attributes::*;

/// BGP Atmic aggregate path attribute
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BgpAtomicAggregate {
    pub value: std::net::IpAddr,
}
impl BgpAtomicAggregate {
    pub fn decode_from(
        _peer: &BgpSessionParams,
        buf: &[u8],
    ) -> Result<BgpAtomicAggregate, BgpError> {
        Ok(BgpAtomicAggregate {
            value: if buf.len() == 0 {
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0))
            } else {
                decode_addr_from(buf)?
            },
        })
    }
}
impl std::fmt::Debug for BgpAtomicAggregate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BgpAtomicAggregate")
            .field("value", &self.value)
            .finish()
    }
}
impl std::fmt::Display for BgpAtomicAggregate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BgpAtomicAggregate {:?}", self.value)
    }
}
impl BgpAttr for BgpAtomicAggregate {
    fn attr(&self) -> BgpAttrParams {
        BgpAttrParams {
            typecode: 6,
            flags: 64,
        }
    }
    fn encode_to(
        &self,
        _peer: &BgpSessionParams,
        buf: &mut [u8],
    ) -> Result<usize, BgpError> {
        encode_addr_to(&self.value, buf)
    }
}

#[cfg(feature = "serialization")]
impl serde::Serialize for BgpAtomicAggregate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.value.to_string().as_str())
    }
}
