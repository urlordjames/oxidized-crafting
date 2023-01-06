use std::net::UdpSocket;
use std::io::Write;

const MAGICAL_LAN_IP: &'static str = "224.0.2.60:4445";

pub fn broadcast_lan() {
	let mut lan_socket = UdpSocket::bind("0.0.0.0:0").unwrap();
	lan_socket.send_to(b"[MOTD]oxidized crafting[/MOTD][AD]25565[/AD]", MAGICAL_LAN_IP).unwrap();
}