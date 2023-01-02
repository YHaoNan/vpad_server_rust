use std::borrow::BorrowMut;
use std::io::stdin;
use std::sync::MutexGuard;

use crate::constants;
use crate::midi_connect::{GLOBAL_MIDI_CONNECTOR, MidiConnector};
use crate::server;

const SLOGAN: &str = r"
____   ______________             .___ __________                __   
\   \ /   /\______   \_____     __| _/ \______   \__ __  _______/  |_ 
 \   Y   /  |     ___/\__  \   / __ |   |       _/  |  \/  ___/\   __\
  \     /   |    |     / __ \_/ /_/ |   |    |   \  |  /\___ \  |  |  
   \___/    |____|    (____  /\____ |   |____|_  /____//____  > |__|  
                           \/      \/          \/           \/        ";

pub async fn startup() {
	print_slogan();
	request_user_to_connect_midi_output_port();
	start_server().await;
}


// ================ Helper Functions ================== //


async fn start_server() {
	let vpad_server = server::VPadServer::bind("0.0.0.0:1236");
	vpad_server.start().await.expect("Cannot start VPadServer.");
}

fn print_slogan() {
	println!("{}", SLOGAN);
	println!("{} -- {}\n\n", constants::SERVER_PLATFORM, constants::SERVER_VERSION);
}

fn request_user_to_connect_midi_output_port() {
	let mut midi_connector = GLOBAL_MIDI_CONNECTOR.lock().unwrap();

	// === print_output_ports_and_select_name
	println!("Available midi output port: ");
	let port_list = midi_connector.port_list().expect("Cannot get midi port list");
	for i in 0..port_list.len() {
		println!("\t{}. {}", i + 1, port_list[i]);
	}
	println!("\n\nPlease select the output port by its number: ");

	let mut index = String::new();
	stdin().read_line(&mut index).expect("Cannot read from stdin");
	let index = index.trim().parse::<usize>().expect("Your input cannot convert to a index");

	let selected_port_name = port_list.get(index - 1).expect(&format!("Cannot get index {}. Please it's not out of bounds", index)).clone();
	// ====== end ======

	println!("Trying to connect to [{}]", selected_port_name);
	midi_connector.connect_port(selected_port_name).expect("Faild to establish the connect!");
	println!("Connection established!");

	if !midi_connector.is_connected() {
		panic!("It seems you already connected to midi output, but the state of MidiConnector is showing that you are now connect successfully! Program exit!");
	}

}


