use super::enums::{DecodeError, MessageType};
use bytes::{BufMut, BytesMut};

const STX: u8 = 0x02;

#[derive(Debug, Default, Clone, Copy)]
pub struct Header {
    stx: u8,
    message_length: u16,
    pub sequence_id: u16,
    pub return_sequence_id: u16,
    pub message_type: MessageType,
    time_stamp: [u8; 8],
    instrument_id: u8,
}

impl Header {
    pub fn new(message_type: MessageType, return_sequence_id: u16, instrument_id: u8) -> Self {
        Self {
            stx: STX,
            message_type,
            return_sequence_id,
            instrument_id,
            ..Default::default()
        }
    }

    pub fn encode(&mut self) -> Option<BytesMut> {
        let mut dst: BytesMut = BytesMut::with_capacity(size_of::<Self>());

        dst.put_u8(self.stx);
        dst.put_u16(self.message_length); // ?
        dst.put_u16(self.sequence_id); // ?
        dst.put_u16(self.return_sequence_id);
        dst.put_u16(self.message_type.into());
        dst.put(&self.time_stamp[..]); // ?
        dst.put_u8(self.instrument_id);

        Some(dst)
    }

    pub fn decode(&mut self, src: &BytesMut) -> Result<(), DecodeError> {
        println!("Header::decoder -> {:?}", src);
        if src[0] != STX {
            return Err(DecodeError::NoSTX);
        }
        self.stx = src[0];
        self.message_length = u16::from_be_bytes(src[1..3].try_into().unwrap());
        self.sequence_id = u16::from_be_bytes(src[3..5].try_into().unwrap());
        self.return_sequence_id = u16::from_be_bytes(src[5..7].try_into().unwrap());
        let num = u16::from_be_bytes(src[7..9].try_into().unwrap());
        self.message_type = MessageType::try_from(num).map_err(|_x| DecodeError::NoSTX)?;
        //self.time_stamp = src[9..17].;
        self.instrument_id = src[17];

        Ok(())
    }
}
