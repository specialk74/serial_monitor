use bytes::{BufMut, BytesMut};

use super::{
    enums::{CarrierOccupancy, SamplePriority, TubeType},
    utils::sample_id_field_length,
};

#[derive(Debug, Clone, Default)]
pub struct UrapTube {
    carrier_occupancy: CarrierOccupancy,
    tube_type: TubeType,
    pub sample_id: String,
    sample_priority: SamplePriority,
    tube_height: u8,
    tube_diameter: u8,
}

impl UrapTube {
    pub fn new(
        carrier_occupancy: CarrierOccupancy,
        tube_type: TubeType,
        sample_id: String,
        sample_priority: SamplePriority,
        tube_height: u8,
        tube_diameter: u8,
    ) -> Self {
        Self {
            carrier_occupancy,
            tube_type,
            sample_id,
            sample_priority,
            tube_height,
            tube_diameter,
        }
    }

    pub fn encode(&mut self) -> Option<BytesMut> {
        let mut dst = BytesMut::with_capacity(0xFFFF);

        dst.put_u8(self.carrier_occupancy as u8);
        dst.put_u8(self.tube_type as u8);
        dst.put(sample_id_field_length(self.sample_id.as_str()));
        dst.put_u8(self.sample_priority as u8);
        dst.put_u8(self.tube_height as u8);
        dst.put_u8(self.tube_diameter as u8);

        Some(dst)
    }
}
