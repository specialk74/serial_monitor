use bytes::BytesMut;
use chrono::{DateTime, Local};
use std::{ascii, io};
use tokio_util::codec::Decoder;

use crate::opt::{CodecOpt, Opt};

pub struct Codec {
    pub name: String,
    codec: fn(src: &BytesMut),
}

fn codec_hex(src: &BytesMut) {
    src.iter().for_each(|x| print!("0x{:02X} ", x));
}

fn codec_dec(src: &BytesMut) {
    src.iter().for_each(|x| print!("{} ", x));
}

fn codec_char(src: &BytesMut) {
    src.iter()
        .for_each(|x| print!("{}", ascii::escape_default(*x)));
}

impl Codec {
    pub fn new(name: String, opt: &Opt) -> Self {
        Self {
            name,
            codec: match opt.codec {
                CodecOpt::Hex => codec_hex,
                CodecOpt::Dec => codec_dec,
                CodecOpt::Char => codec_char,
            },
        }
    }
}

impl Decoder for Codec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let current_local: DateTime<Local> = Local::now();
        print!("{} | {} | ", self.name, current_local.format("%X:%6f"));
        (self.codec)(src);
        println!();
        src.clear();
        Ok(None)
    }
}
