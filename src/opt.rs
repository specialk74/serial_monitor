use std::io;
use std::result::Result as StdResult;
use structopt::StructOpt;
use tokio_serial::{DataBits, FlowControl, Parity, StopBits};

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "serial-monitor")]
pub struct Opt {
    /// Filter based on name of port
    #[structopt(short, long)]
    pub port: Option<String>,

    /// Baud rate to use
    #[structopt(short, long, default_value = "9600")]
    pub baud: u32,

    /// Turn on debugging
    #[structopt(short, long)]
    pub debug: bool,

    /// List USB serial devices which are currently connected
    #[structopt(short, long)]
    pub list: bool,

    /// Like list, but only prints the name of the port that was found.
    /// This is useful for using from scripts or makefiles.
    #[structopt(short, long)]
    pub find: bool,

    /// Turn on verbose messages
    #[structopt(short, long)]
    pub verbose: bool,

    /// Filter based on Vendor ID (VID)
    #[structopt(long)]
    pub vid: Option<String>,

    /// Filter based on Product ID (PID)
    #[structopt(long)]
    pub pid: Option<String>,

    /// Filter based on manufacturer name
    #[structopt(short, long)]
    pub manufacturer: Option<String>,

    /// Filter based on serial number
    #[structopt(short, long)]
    pub serial: Option<String>,

    /// Filter based on product name
    #[structopt(long)]
    pub product: Option<String>,

    /// Return the index'th result
    #[structopt(long)]
    pub index: Option<usize>,

    /// Parity checking (none, odd, even)
    #[structopt(long, default_value = "none")]
    pub parity: ParityOpt,

    /// Stop bits (1, 2)
    #[structopt(long, default_value = "1")]
    pub stopbits: usize,

    /// Flow control (none, software, hardware)
    #[structopt(long, default_value = "none")]
    pub flow: FlowControlOpt,

    /// Data bits (5, 6, 7, 8)
    #[structopt(long, default_value = "8")]
    pub databits: usize,

    /// Byte codec (Hex, Decimal, Char)
    #[structopt(long, default_value = "hex")]
    pub codec: CodecOpt,
}

struct DataBitsOpt(DataBits);

impl TryFrom<usize> for DataBitsOpt {
    type Error = io::Error;

    fn try_from(value: usize) -> StdResult<Self, io::Error> {
        match value {
            5 => Ok(Self(DataBits::Five)),
            6 => Ok(Self(DataBits::Six)),
            7 => Ok(Self(DataBits::Seven)),
            8 => Ok(Self(DataBits::Eight)),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "databits out of range",
            )),
        }
    }
}

/// Flow control modes
#[derive(Clone, Copy, Debug, StructOpt, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum FlowControlOpt {
    /// No flow control.
    None,
    /// Flow control using XON/XOFF bytes.
    Software,
    /// Flow control using RTS/CTS signals.
    Hardware,
}

impl From<FlowControlOpt> for FlowControl {
    fn from(opt: FlowControlOpt) -> Self {
        match opt {
            FlowControlOpt::None => FlowControl::None,
            FlowControlOpt::Software => FlowControl::Software,
            FlowControlOpt::Hardware => FlowControl::Hardware,
        }
    }
}

#[derive(Clone, Copy, Debug, StructOpt, strum::EnumString, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum CodecOpt {
    Hex,
    Dec,
    Char,
}

#[derive(Clone, Copy, Debug, StructOpt, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum ParityOpt {
    /// No parity bit.
    None,
    /// Parity bit sets odd number of 1 bits.
    Odd,
    /// Parity bit sets even number of 1 bits.
    Even,
}

impl From<ParityOpt> for Parity {
    fn from(opt: ParityOpt) -> Self {
        match opt {
            ParityOpt::None => Parity::None,
            ParityOpt::Odd => Parity::Odd,
            ParityOpt::Even => Parity::Even,
        }
    }
}

struct StopBitsOpt(StopBits);

impl TryFrom<usize> for StopBitsOpt {
    type Error = io::Error;

    fn try_from(value: usize) -> StdResult<Self, io::Error> {
        match value {
            1 => Ok(Self(StopBits::One)),
            2 => Ok(Self(StopBits::Two)),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "stopbits out of range",
            )),
        }
    }
}
