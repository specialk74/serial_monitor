use bytes::{BufMut, BytesMut};
//use message_parser_macros::MessageParsers;

use crate::atellica::{
    enums::{AddQueueCommandStatusValues, DecodeError, InterfacePositionIndex, MessageType},
    info::Info,
    utils::sample_id_field_length,
};

#[derive(Debug, Clone, Default)]
pub struct AddQueueResponse {
    interface_position_index: InterfacePositionIndex,
    sample_id: String,
    command_status: AddQueueCommandStatusValues,
}

impl AddQueueResponse {
    pub fn new(
        interface_position_index: InterfacePositionIndex,
        sample_id: String,
        command_status: AddQueueCommandStatusValues,
    ) -> Self {
        Self {
            interface_position_index,
            sample_id,
            command_status,
        }
    }

    pub fn from_bytes(src: &BytesMut) -> Result<Self, DecodeError> {
        let interface_position_index =
            InterfacePositionIndex::try_from(src[18]).map_err(|_x| DecodeError::NoSTX)?;
        let sample_id = String::from("");
        Ok(Self {
            interface_position_index,
            sample_id,
            command_status: AddQueueCommandStatusValues::Ok,
        })
    }
}

impl Info for AddQueueResponse {
    fn encode(&mut self) -> Option<BytesMut> {
        let mut dst = BytesMut::with_capacity(0xFFFF);

        dst.put_u8(self.interface_position_index as u8);
        dst.put(sample_id_field_length(self.sample_id.as_str()));
        dst.put_u8(self.command_status as u8);

        Some(dst)
    }

    fn get_message_type(&self) -> MessageType {
        MessageType::AddQueueResponse
    }
}
