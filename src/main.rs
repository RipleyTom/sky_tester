use std::time::Duration;

fn sniff_skylander() -> libusb::Result<()> {
    let context = libusb::Context::new().unwrap();
    let res = context.open_device_with_vid_pid(0x1430, 0x0150);
    if let None = res {
        return Err(libusb::Error::NoDevice);
    }
    let device = res.unwrap();

    let mut buf = [0u8; 32];

    let mut loop_num = 0;
    loop {
        loop_num += 1;
        match loop_num {
            4 => {
                println!("Sending reset!");
                let reset_command: [u8; 32] = [0x52, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00];
                device.write_control(0x21, 0x09, 0x200, 0, &reset_command, Duration::from_secs(1))?;
            }
            5 => {
                println!("Sending activate!");
                let activate_command: [u8; 32] = [0x41, 0x01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00];
                device.write_control(0x21, 0x09, 0x200, 0, &activate_command, Duration::from_secs(1))?;
            }
            v if v%10 == 0 => {
                println!("Sending Query!");
                let query_command: [u8; 32] = ['Q' as u8, 0x21, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00];
                device.write_control(0x21, 0x09, 0x200, 0, &query_command, Duration::from_secs(1))?;
            }
            _ => {}
        }

        let res = device.read_interrupt(0x81, &mut buf, Duration::from_secs(1))?;
        assert_eq!(res, 32);

        // if buf[0] != 0x53 {
        //     println!("{:X?}", buf);
        // } else if loop_num % 10 == 9 {
            println!("{:X?}", buf);
        // }
        
    }
}

fn main() {
    if let Err(e) = sniff_skylander() {
        println!("Error: {}", e);
    }
}
