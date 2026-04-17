use std::{
    sync::mpsc::{self, Receiver},
    time::Duration,
};

pub fn test_mpsc_channels() {
    let (transmitter, reciever) = mpsc::channel::<u8>();

    let send_result = transmitter.send(100);
    println!("Send status: {}", send_result.is_ok());

    transmitter.send(152);

    let receive_result = reciever.recv_timeout(Duration::from_millis(300));
    println!("Receive result is: {}", receive_result.is_ok());
    println!("Receive result is: {}", receive_result.unwrap());

    let receive_result = reciever.recv_timeout(Duration::from_millis(300));
    println!("Receive result is: {}", receive_result.is_ok());
    println!("Receive result is: {}", receive_result.unwrap());
}
