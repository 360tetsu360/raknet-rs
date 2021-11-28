use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use crate::packets::Packet;
use std::{convert::TryInto, io::Result, net::SocketAddr};
pub struct NewIncomingConnection {
    server_address: SocketAddr,
    system_address: [SocketAddr; 20],
    request_timestamp: u64,
    accepted_timestamp: u64,
}

impl Packet for NewIncomingConnection {
    const ID:u8 = 0x13;
    fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            server_address: cursor.read_address()?,
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
    fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_address(self.server_address)?;
        for address in self.system_address {
            cursor.write_address(address)?;
        }
        cursor.write_u64(self.request_timestamp, Endian::Big)?;
        cursor.write_u64(self.accepted_timestamp, Endian::Big)?;
        Ok(cursor.get_raw_payload())
    }
}
