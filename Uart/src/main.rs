use std::io::Read;
use std::thread;
use std::time::Duration;

fn main() {
    let mut port = serialport::new("/dev/tty.usbserial-FT9LIY4X", 115_200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    let mut rx_port = port.try_clone().expect("unable to clone the port");

    thread::spawn(move || unsafe {
        println!("spawning the uart rx thread...");
        thread::sleep(Duration::from_secs(3));
        loop {
            let mut serial_buf: Vec<u8> = vec![0; 32];
            rx_port
                .read(serial_buf.as_mut_slice())
                .expect("Found no data!");
            let data_str = String::from_utf8(serial_buf).expect("unable to parse the rx data");
            println!("rx-> {}", data_str);
            thread::sleep(Duration::from_secs(1));
        }
    });

    let mut count: i32 = 0;
    loop {
        let data = format!("{}", count);
        println!("Tx: {}", data);
        let output = data.as_bytes();
        port.write(output).expect("Write failed!");
        thread::sleep(Duration::from_secs(1));
        count += 1;
    }
}
