use tokio::net::UdpSocket;

const MAGICAL_LAN_IP: &'static str = "224.0.2.60:4445";
const MOTD: &[u8]= b"[MOTD]oxidized crafting[/MOTD][AD]25565[/AD]";

pub async fn broadcast_lan() {
	let lan_socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();

	tokio::spawn(async move {
		loop {
			tokio::time::sleep(core::time::Duration::from_millis(1500)).await;
			lan_socket.send_to(MOTD, MAGICAL_LAN_IP).await.unwrap();
		}
	});
}
