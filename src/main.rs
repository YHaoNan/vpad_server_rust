#![allow(dead_code)]

extern crate core;

mod cmd;
mod server;
mod message;
mod constants;
mod midi_connect;
mod arp_handler;
mod pulse_generator;
mod circle_container;
mod pitch_wheel;
mod message_codec;
mod chord_handler;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs-config.yaml", Default::default()).unwrap();
    log::info!("log4rs initialized!");
    log::info!("starting command line client!");
    cmd::startup().await;
}