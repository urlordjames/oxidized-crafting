#[derive(Debug, Clone, Copy)]
pub enum State {
	Handshake,
	Status,
	Login
}
