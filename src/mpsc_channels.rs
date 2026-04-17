use std::sync::mpsc;

pub fn test_mpsc_channels() {
    let (transmitter, reciever) = mpsc::channel::<u8>();

    let send_result = transmitter.send(100);
    println!("Send status: {}", send_result.is_ok());

    transmitter.send(152);
}
