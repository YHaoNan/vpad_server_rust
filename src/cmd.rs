use std::io::stdin;
use std::net::{IpAddr};
use std::str::FromStr;
use std::sync::MutexGuard;
use std::env;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use crate::constants;
use crate::midi_connect::{GLOBAL_CTL_CONNECTOR, GLOBAL_MIDI_CONNECTOR, MidiConnector};
use crate::server;
use clap::Parser;

#[derive(Parser)]
#[command(name = "VPadCore")]
#[command(author = "Yudoge. <yohahaonan@gmail.com>")]
#[command(version = "1.0")]
#[command(author, version)]
struct CoreCli {
	#[arg(short)]
	instrument_midi_port: String,
	#[arg(short)]
	control_midi_port: String,
	#[arg(short)]
	log_level: Option<String>
}

const SLOGAN: &str = r"
____   ______________             .___ __________                __   
\   \ /   /\______   \_____     __| _/ \______   \__ __  _______/  |_ 
 \   Y   /  |     ___/\__  \   / __ |   |       _/  |  \/  ___/\   __\
  \     /   |    |     / __ \_/ /_/ |   |    |   \  |  /\___ \  |  |  
   \___/    |____|    (____  /\____ |   |____|_  /____//____  > |__|  
                           \/      \/          \/           \/        ";

pub async fn startup() {
	if env::args().len() > 1 {
		// core mode
		println!("core mode!");
		let cli = CoreCli::parse();
		let midi_connector = GLOBAL_MIDI_CONNECTOR.lock().unwrap();
		let ctl_connector = GLOBAL_CTL_CONNECTOR.lock().unwrap();
		println!("Trying to connect to {}", &cli.instrument_midi_port);
		connect_to_a_midi_port(midi_connector, cli.instrument_midi_port);
		println!("Trying to connect to {}", &cli.control_midi_port);
		connect_to_a_midi_port(ctl_connector, cli.control_midi_port);
		start_server().await;
	} else {
		// standalone mode
		print_slogan();
		request_user_to_connect_midi_output_port();
		start_server().await;
	}
}


// ================ Helper Functions ================== //


async fn start_server() {
	let vpad_server = server::VPadServer::bind(IpAddr::from_str("0.0.0.0").expect(""), 1236);
	vpad_server.start().await.expect("Cannot start VPadServer.");
}

fn print_slogan() {
	println!("{}", SLOGAN);
	println!("{} -- {}\n\n", constants::SERVER_PLATFORM, constants::SERVER_VERSION);
}

fn connect_to_a_midi_port(mut connector: MutexGuard<MidiConnector>, port: String) {
	connector.connect_port(port).expect("faild to connect");
	println!("Connection established!");
	if !connector.is_connected() {
		panic!("It seems you already connected to midi output, but the state of MidiConnector is showing that you are now connect successfully! Program exit!");
	}
}

fn select_a_port_and_connect(connector: MutexGuard<MidiConnector>, port_list: &Vec<String>) {
	let mut index = String::new();
	stdin().read_line(&mut index).expect("Cannot read from stdin");
	let index = index.trim().parse::<usize>().expect("Your input cannot convert to a index");
	let selected_port_name = port_list.get(index - 1).expect(&format!("Cannot get index {}. Please it's not out of bounds", index)).clone();
	println!("Trying to connect to [{}]", selected_port_name);
	connect_to_a_midi_port(connector, selected_port_name);
}

fn request_user_to_connect_midi_output_port() {
	let midi_connector = GLOBAL_MIDI_CONNECTOR.lock().unwrap();
	let ctl_connector = GLOBAL_CTL_CONNECTOR.lock().unwrap();

	// === print_output_ports_and_select_name
	println!("Available midi output port: ");
	let port_list = MidiConnector::port_list().expect("Cannot get midi port list");
	for i in 0..port_list.len() {
		println!("\t{}. {}", i + 1, port_list[i]);
	}

	println!("\n\nChoose instrument midi device: ");
	select_a_port_and_connect(midi_connector, &port_list);

	println!("\n\nChoose control midi device: ");
	select_a_port_and_connect(ctl_connector, &port_list);

	println!("\n\nAll Settings done! Enjoy it~");

	print_qrcode();
}


/// 遍历每个网络接口，获取所有合法ip地址
/// 	1. 必须是ipv4
/// 	2. 必须不能是loopback
/// 	3. 如果一个接口上有多个ip，取第一个符合的ip
fn get_all_vaild_ip_addresses() -> Vec<String> {
	NetworkInterface::show().expect("cannot get network interfaces").iter().filter_map(|iface| {
		let addrs = &iface.addr;
		for addr in addrs {
			let ip = addr.ip();
			if ip.is_ipv4() && !ip.is_loopback() {
				return Some(ip.to_string())
			}
		}
		None
	}).collect()
}

fn print_qrcode() {
	println!("There is your qrcode: ");
	let qrcontent = get_all_vaild_ip_addresses().join(";");
	if qrcontent.is_empty() { panic!("it seems there's no any network interface on your computer. so ... panic!"); }
	qr2term::print_qr(qrcontent).expect("cannot print qrcode");
}

