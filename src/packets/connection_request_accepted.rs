use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::net::{IpAddr, Ipv6Addr};
use std::{convert::TryInto, io::Result, net::SocketAddr};

pub struct ConnectionRequestAccepted {
    pub client_address: SocketAddr,
    system_address: [SocketAddr; 20],
    pub request_timestamp: u64,
    pub accepted_timestamp: u64,
}

impl ConnectionRequestAccepted {
    pub fn new(
        client_address: SocketAddr,
        request_timestamp: u64,
        accepted_timestamp: u64,
    ) -> Self {
        Self {
            client_address,
            system_address: {
                let address = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), 0);
                [address; 20]
            },
            request_timestamp,
            accepted_timestamp,
        }
    }
    pub fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            client_address: cursor.read_address()?,
            system_address: {
                let mut addresses = vec![];
                for _ in 1..20 {
                    addresses.push(cursor.read_address().unwrap());
                }
                (*addresses.as_slice()).try_into().unwrap()
            },
            request_timestamp: cursor.read_u64(Endian::Big)?,
            accepted_timestamp: cursor.read_u64(Endian::Big)?,
        })
    }
    pub fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_address(self.client_address)?;
        cursor.write_u16(0, Endian::Big)?;
        for address in self.system_address {
            cursor.write_address(address)?;
        }
        cursor.write_u64(self.request_timestamp, Endian::Big)?;
        cursor.write_u64(self.accepted_timestamp, Endian::Big)?;
        Ok(cursor.get_raw_payload())
    }
}
