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

#[tokio::main]
async fn main() {
    cmd::startup().await;
}