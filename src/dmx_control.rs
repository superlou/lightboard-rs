use std::sync::mpsc;
use std::{thread, time};
use dmx::{self, DmxTransmitter};

pub fn update(recv: mpsc::Receiver<Vec<u8>>) {
    let mut dmx_port = match dmx::open_serial("/dev/ttyUSB0") {
        Ok(port) => port,
        Err(_) => {
            println!("Unable to connect to serial port!");
            return
        },
    };

    let mut data = vec![];

    loop {
        if let Ok(new_data) = recv.try_recv() {
            data = new_data;
        }

        dmx_port.send_dmx_packet(&data).unwrap();
        thread::sleep(time::Duration::new(0, 20_000_000));
    }
}
