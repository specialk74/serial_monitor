use bytes::{BufMut, BytesMut};

pub fn sample_id_field_length(sample_id: &str) -> BytesMut {
    let mut dst = BytesMut::with_capacity(sample_id.len() + 1);

    dst.put_u8(sample_id.len() as u8);
    dst.put(sample_id.as_bytes());

    dst
}
