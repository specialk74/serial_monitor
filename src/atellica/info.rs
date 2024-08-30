use bytes::BytesMut;

use super::enums::MessageType;

pub trait Info {
    fn encode(&mut self) -> Option<BytesMut> {
        None
    }
    fn get_message_type(&self) -> MessageType {
        MessageType::NoMessageType
    }
    fn response_message_type(&self) -> Option<MessageType> {
        None
    }
}
