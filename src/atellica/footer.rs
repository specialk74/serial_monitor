use bytes::{BufMut, BytesMut};

use super::enums::DecodeError;

const ETX: u8 = 0x03;

#[derive(Debug, Clone, Default)]
pub struct Footer {
    crc: u16,
    etx: u8,
}

impl Footer {
    pub fn new() -> Self {
        Self { crc: 0, etx: ETX }
    }
    pub fn encode(&mut self) -> Option<BytesMut> {
        let mut dst: BytesMut = BytesMut::with_capacity(size_of::<Self>());

        dst.put_u16(self.crc);
        dst.put_u8(self.etx);

        Some(dst)
    }

    pub fn decode(&mut self, src: &BytesMut) -> Result<(), DecodeError> {
        println!("{:?}", src);
        Ok(())
    }
}
