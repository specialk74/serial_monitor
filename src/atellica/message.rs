use bytes::{BufMut, BytesMut};

use super::{
    enums::MessageType,
    footer::Footer,
    header::Header,
    info::Info,
    messages::add_queue::{
        add_queue_request::AddQueueRequest, add_queue_response::AddQueueResponse,
    },
    payload::*,
};

#[derive(Debug, Default)]
pub struct Message {
    header: Header,
    message_body: Payload,
    footer: Footer,
}

impl_payload! { AddQueueRequest, AddQueueResponse }

impl Message {
    pub fn new(return_sequence_id: u16, instrument_id: u8, message_body: Payload) -> Self {
        Self {
            header: Header::new(
                message_body.get_message_type(),
                return_sequence_id,
                instrument_id,
            ),
            message_body,
            footer: Footer::new(),
        }
    }

    fn set_sequence_id(&mut self, sequence_id: u16) {
        self.header.sequence_id = sequence_id;
    }

    fn is_my_response(&self, resp: Message) -> bool {
        if let Some(message_type) = self.message_body.response_message_type() {
            return self.header.sequence_id == resp.header.return_sequence_id
                && message_type == resp.message_body.get_message_type();
        }
        false
    }

    pub fn need_response(&self) -> bool {
        self.message_body.response_message_type().is_some()
    }
}

impl Info for Message {
    fn encode(&mut self) -> Option<BytesMut> {
        let mut dst = BytesMut::with_capacity(0xFFFF);

        dst.put(self.header.encode()?);
        dst.put(self.message_body.encode()?);
        dst.put(self.footer.encode()?);

        Some(dst)
    }
}

#[cfg(test)]
mod tests {
    use crate::atellica::{
        enums::{
            AddQueueCommandStatusValues, CarrierOccupancy, InterfacePositionIndex, SamplePriority,
            TubeType,
        },
        urap_tube::UrapTube,
    };

    use super::*;

    #[test]
    fn message2_test() {
        let tube = UrapTube::new(
            CarrierOccupancy::CappedTube,
            TubeType::Greiner,
            "SampleId".to_string(),
            SamplePriority::Routine,
            0,
            0,
        );

        let instrument_id = 10;
        let interface_position_index = InterfacePositionIndex::IP1;

        let mut request = Message::new(
            0,
            instrument_id,
            Payload::AddQueueRequest(AddQueueRequest::new(interface_position_index, &tube)),
        );
        request.set_sequence_id(0x1234);
        let encode_request = request.encode().expect("");

        println!("request.encode: {:?}", encode_request);

        let mut response = Message::new(
            request.header.sequence_id,
            instrument_id,
            Payload::AddQueueResponse(AddQueueResponse::new(
                interface_position_index,
                tube.sample_id.clone(),
                AddQueueCommandStatusValues::Ok,
            )),
        );
        response.set_sequence_id(0x5678);

        let encode_response = response.encode().expect("");
        println!("response.encode(): {:?}", encode_response);
        println!(
            "request.is_my_response(response): {}",
            request.is_my_response(response)
        );

        let msg_request_decoded = message_factory(&encode_request);
        println!("msg_request_decoded: {:?}", msg_request_decoded);

        let msg_response_decoded = message_factory(&encode_response);
        println!("msg_response_decoded: {:?}", msg_response_decoded);
    }
}
