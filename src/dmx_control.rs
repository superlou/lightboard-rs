use std::sync::mpsc;
use std::{thread, time};
use dmx::{self, DmxTransmitter};

pub fn update(recv: mpsc::Receiver<[u8; 8]>) {
    let mut dmx_port = match dmx::open_serial("/dev/ttyUSB0") {
        Ok(port) => port,
        Err(_) => {
            println!("Unable to connect to serial port!");
            return
        },
    };

    let mut data = [0, 0, 0, 255, 200, 0, 200, 0];

    loop {
        match recv.try_recv() {
            Ok(new_data) => data = new_data,
            Err(_) => {},
        };

        dmx_port.send_dmx_packet(&data).unwrap();
        thread::sleep(time::Duration::new(0, 20_000_000));
    }
}
