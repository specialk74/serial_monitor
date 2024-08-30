pub mod ports {
    use crate::error::ProgramError;
    use crate::opt::Opt;
    use regex_lite::Regex;
    use std::str;
    use tokio_serial::{SerialPortInfo, SerialPortType, UsbPortInfo};

    #[cfg(target_os = "macos")]
    fn map_port_name(port_name: &str) -> String {
        // available_ports returns /dev/tty.* rather than /dev/cu.*
        // /dev/tty.* are designed for incoming serial connections and will block
        // until DCD is set.
        // /dev/cu.* are designed for outgoing serial connections and don't block,
        // so we change /dev/tty.* to /dev/cu.* since this program is primarily
        // used for outgoing connections.
        if port_name.starts_with("/dev/tty.") {
            port_name.replace("/dev/tty.", "/dev/cu.")
        } else {
            String::from(port_name)
        }
    }

    // Returns a list of the available ports (for macos)
    #[cfg(target_os = "macos")]
    fn available_ports() -> Result<Vec<SerialPortInfo>, ProgramError> {
        Ok(tokio_serial::available_ports()
            .map_err(|x| ProgramError::SerialPortError(x))?
            .into_iter()
            .map(|mut port| {
                port.port_name = map_port_name(&port.port_name);
                port
            })
            .collect())
    }

    // Returns a list of the available ports (for everything but macos)
    #[cfg(not(target_os = "macos"))]
    fn available_ports() -> Result<Vec<SerialPortInfo>, ProgramError> {
        Ok(tokio_serial::available_ports().map_err(|e| ProgramError::SerialPortError(e))?)
    }

    // Checks to see if a string matches a pattern used for filtering.
    fn matches(str: &str, pattern: Option<String>, opt: &Opt) -> bool {
        let result = match pattern.clone() {
            Some(pattern) => {
                let re = Regex::new(&pattern).unwrap();
                re.is_match(str)
            }
            None => {
                // If no pattern is specified, then we consider that
                // a match has taken place.
                true
            }
        };
        if opt.debug {
            println!(
                "matches(str:{:?}, pattern:{:?}) -> {:?}",
                str, pattern, result
            );
        }
        result
    }

    // Similar to matches but checks to see if an Option<String> matches a pattern.
    fn matches_opt(str: Option<String>, pattern: Option<String>, opt: &Opt) -> bool {
        if let Some(str) = str {
            matches(&str, pattern, opt)
        } else {
            // If no pattern was specified, then we don't care if there was a string
            // supplied or not. But if we're looking for a particular patterm, then
            // it needs to match.
            let result = pattern.is_none();
            if opt.debug {
                println!(
                    "matches_opt(str:{:?}, pattern:{:?}) -> {:?}",
                    str, pattern, result
                );
            }
            result
        }
    }

    // Checks to see if a serial port matches the filtering criteria specified on the command line.
    fn usb_port_matches(port: &SerialPortInfo, opt: &Opt) -> bool {
        if let SerialPortType::UsbPort(info) = &port.port_type {
            if matches(&port.port_name, opt.port.clone(), opt)
                && matches(&format!("{:04x}", info.vid), opt.vid.clone(), opt)
                && matches(&format!("{:04x}", info.pid), opt.pid.clone(), opt)
                && matches_opt(info.manufacturer.clone(), opt.manufacturer.clone(), opt)
                && matches_opt(info.serial_number.clone(), opt.serial.clone(), opt)
                && matches_opt(info.product.clone(), opt.product.clone(), opt)
            {
                return true;
            }
        }
        false
    }

    fn filtered_ports(opt: &Opt) -> Result<Vec<SerialPortInfo>, ProgramError> {
        let mut ports: Vec<SerialPortInfo> = available_ports()?
            .into_iter()
            .filter(|info| usb_port_matches(&info, opt))
            .collect();
        ports.sort_by(|a, b| a.port_name.cmp(&b.port_name));
        if let Some(index) = opt.index {
            if index < ports.len() {
                Ok(vec![ports[index].clone()])
            } else {
                Err(ProgramError::NoPortFound)
            }
        } else if ports.is_empty() {
            Err(ProgramError::NoPortFound)
        } else {
            Ok(ports)
        }
    }

    fn filtered_port(opt: &Opt) -> Result<SerialPortInfo, ProgramError> {
        Ok(filtered_ports(opt)?[0].clone())
    }

    // Formats the USB Port information into a human readable form.
    fn extra_usb_info(info: &UsbPortInfo) -> String {
        let mut output = String::new();
        output = output + &format!(" {:04x}:{:04x}", info.vid, info.pid);
        let mut extra_items = Vec::new();

        if let Some(manufacturer) = &info.manufacturer {
            extra_items.push(format!("manufacturer '{}'", manufacturer));
        }
        if let Some(serial) = &info.serial_number {
            extra_items.push(format!("serial '{}'", serial));
        }
        if let Some(product) = &info.product {
            extra_items.push(format!("product '{}'", product));
        }
        if !extra_items.is_empty() {
            output += " with ";
            output += &extra_items.join(" ");
        }
        output
    }

    // Lists all of the USB serial ports which match the filtering criteria.
    pub fn list_ports(opt: &Opt) -> Result<(), ProgramError> {
        for port in filtered_ports(opt)? {
            if let SerialPortType::UsbPort(info) = &port.port_type {
                println!(
                    "USB Serial Device{} found @{}",
                    extra_usb_info(&info),
                    port.port_name
                );
            } else {
                println!("Serial Device found @{}", port.port_name);
            }
        }
        Ok(())
    }

    // Returns the first port which matches the filtering criteria.
    pub fn find_first_port(opt: &Opt) -> Result<String, ProgramError> {
        Ok(filtered_port(opt)?.port_name)
    }

    // Returns the first port which matches the filtering criteria.
    pub fn find_ports(opt: &Opt) -> Result<Vec<String>, ProgramError> {
        Ok(filtered_ports(opt)?
            .iter()
            .map(|x| x.port_name.clone())
            .collect())
    }
}
