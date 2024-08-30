use codec::Codec;
use error::ProgramError;
use futures::StreamExt;
use interface::ports;
use opt::Opt;
use std::process::exit;
use structopt::StructOpt;
use tokio_serial::SerialPortBuilderExt;
use tokio_util::codec::Decoder;

mod atellica;

mod codec;
mod error;
mod interface;
mod opt;

fn handle_request(path: &String, opt: &Opt) -> tokio::task::JoinHandle<()> {
    let mut port = match tokio_serial::new(path, opt.baud).open_native_async() {
        Err(x) => panic!("Error: {:?}", x),
        Ok(x) => x,
    };

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    let mut reader = Codec::new(path.clone(), &opt).framed(port);
    tokio::task::spawn(async move {
        loop {
            reader.next().await;
        }
    })
}

fn handle_opt(opt: &Opt) -> Result<(), ProgramError> {
    if opt.verbose {
        println!("{:#?}", opt);
    }

    if opt.list {
        ports::list_ports(&opt)?;
        exit(0);
    }

    if opt.find {
        println!("{}", ports::find_first_port(&opt)?);
        exit(0);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ProgramError> {
    let opt = Opt::from_args();

    handle_opt(&opt)?;

    let tasks: Vec<_> = ports::find_ports(&opt)
        .unwrap()
        .into_iter()
        .map(|f| handle_request(&f, &opt))
        .collect();

    futures::future::join_all(tasks).await;

    Ok(())
}
