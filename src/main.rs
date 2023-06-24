use tokio::net::{TcpListener, TcpStream};

mod lan;
use lan::broadcast_lan;

mod packet;
use packet::{Packet, write_packet};
use packet::packet_data::{read_long, write_long};

mod handshake;
use handshake::Handshake;

mod state;
use state::{State, LoginState, PlayerInfo};

mod text;
use text::Text;

mod status;
use status::StatusResponse;

mod login;
use login::{LoginStart, LoginSuccess};

pub mod position;

mod play;
use play::Login;

pub const GAME_VERSION: &str = "1.20.1";
pub const PROTOCOL_VERSION: u64 = 763;

#[tokio::main]
async fn main() {
	let listener = TcpListener::bind("0.0.0.0:25565").await.unwrap();

	broadcast_lan().await;

	loop {
		if let Ok((mut stream, _)) = listener.accept().await {
			tokio::spawn(async move {
				handle_client(&mut stream).await;
			});
		}
	}
}

async fn handle_client(stream: &mut TcpStream) {
	println!("new connection");

	let mut state = State::Handshake;

	loop {
		let mut packet = Packet::read(stream).await;
		println!("{:?}", packet);

		match (packet.id, &state) {
			(0x00, State::Handshake) => {
				let handshake = Handshake::read(&mut packet).await;
				println!("{:?}", handshake);

				state = handshake.next_state;
			},
			(0x00, State::Status) => {
				let resp = StatusResponse::default();
				resp.write(stream).await;
			},
			(0x01, State::Status) => {
				let ping_data = read_long(&mut packet.data).await;

				let mut pong_buf = vec![];
				write_long(&mut pong_buf, ping_data).await;

				write_packet(stream, 0x01, pong_buf).await;
				return;
			},
			(0x00, State::Login(LoginState::PostHandshake)) => {
				let login_start = LoginStart::read(&mut packet).await;
				println!("{:?}", login_start);

				state = State::Play(PlayerInfo {
					name: login_start.name,
					uuid: login_start.uuid
				});

				let login_success = LoginSuccess {
					uuid: login_start.uuid.unwrap(),
					username: state.username().unwrap()
				};

				login_success.write(stream).await;

				let login = Login::default();
				login.write(stream).await;
			},
			(_, State::Play(_)) => {
				let reason = Text::from("TODO: implement play state");

				write_packet(stream, 0x17, serde_json::to_vec(&reason).unwrap()).await;
				return;
			},
			(id, state) => todo!("TODO: implement packet with id {:x} in state {:?}", id, state)
		}
	}
}
