use bytes::{BufMut, BytesMut};

use crate::atellica::{
    enums::{DecodeError, InterfacePositionIndex, MessageType},
    info::Info,
    urap_tube::UrapTube,
};

#[derive(Debug, Clone, Default)]
pub struct AddQueueRequest {
    interface_position_index: InterfacePositionIndex,
    tube: UrapTube,
}

impl AddQueueRequest {
    pub fn new(interface_position_index: InterfacePositionIndex, tube: &UrapTube) -> Self {
        Self {
            interface_position_index,
            tube: tube.clone(),
        }
    }

    pub fn from_bytes(src: &BytesMut) -> Result<Self, DecodeError> {
        let interface_position_index =
            InterfacePositionIndex::try_from(src[18]).map_err(|_x| DecodeError::NoSTX)?;

        let tube = UrapTube::default();
        Ok(Self {
            interface_position_index,
            tube: tube.clone(),
        })
    }
}

impl Info for AddQueueRequest {
    fn encode(&mut self) -> Option<BytesMut> {
        let mut dst = BytesMut::with_capacity(0xFFFF);

        dst.put_u8(self.interface_position_index as u8);
        dst.put(self.tube.encode()?);

        Some(dst)
    }

    fn get_message_type(&self) -> MessageType {
        MessageType::AddQueueRequest
    }

    fn response_message_type(&self) -> Option<MessageType> {
        Some(MessageType::AddQueueResponse)
    }
}
