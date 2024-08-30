#[macro_use]
#[macro_export]
macro_rules! impl_payload {
    ($($var:ident),*) => {
        #[derive(Debug, Default)]
        enum Payload {
            #[default]
            NoMessage,
            $(
            $var($var),
        )*}

        impl Info for Payload {
            fn encode(&mut self) -> Option<BytesMut> {
                match self {
                    $(Payload::$var(msg) => msg.encode(),)*
                    _ => panic!("Nop"),
                }
            }

            fn get_message_type(&self) -> MessageType {
                match self {
                    $(Payload::$var(msg) => msg.get_message_type(),)*
                    _ => panic!("Nop"),
                }
            }

            fn response_message_type(&self) -> Option<MessageType> {
                match self {
                    $(Payload::$var(msg) => msg.response_message_type(),)*
                    _ => panic!("Nop"),
                }
            }
        }

        fn message_factory(src: &BytesMut) -> Option<Message> {
            let mut msg = Message::default();
            msg.header.decode(src).ok()?;
            msg.message_body = match msg.header.message_type {
                $(MessageType::$var => Payload::$var($var::from_bytes(src).expect("msg")),)*
                _ => panic!("NoMessageType"),
            };
            msg.footer.decode(src).ok()?;

            Some(msg)
        }
    }
}
pub use impl_payload;
