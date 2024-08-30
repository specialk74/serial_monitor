use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive, Default)]
#[repr(u8)]
pub enum InterfacePositionIndex {
    #[default]
    IP0,
    IP1,
}

#[derive(Debug, Clone, Copy, Default, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CarrierOccupancy {
    #[default]
    EmptyCarrier,
    UncappedTube,
    CappedTube,
}

#[derive(Debug, Clone, Copy, Default, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TubeType {
    #[default]
    Greiner,
}

#[derive(Debug, Clone, Copy, Default, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum SamplePriority {
    #[default]
    Undefined,
    Routine,
    STAT,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum MessageType {
    #[default]
    NoMessageType,
    AddQueueRequest = 0x0405,
    AddQueueResponse = 0x0406,
}

#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive, Default)]
#[repr(u8)]
pub enum AddQueueCommandStatusValues {
    #[default]
    Ok,
}

#[derive(Debug)]
pub enum DecodeError {
    MessageTypeError,
    NoSTX,
}
