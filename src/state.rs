#[derive(Debug)]
pub enum LoginState {
	PostHandshake
}

#[derive(Debug)]
pub struct PlayerInfo {
	pub name: String,
	pub uuid: Option<u128>
}

#[derive(Debug)]
pub enum State {
	Handshake,
	Status,
	Login(LoginState),
	Play(PlayerInfo)
}

impl State {
	pub fn username(&self) -> Option<&str> {
		match self {
			State::Play(info) => Some(&info.name),
			_ => None
		}
	}
}
