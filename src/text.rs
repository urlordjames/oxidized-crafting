use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Text {
	text: String
}

impl<S: Into<String>> From<S> for Text {
	fn from(text: S) -> Self {
		Self {
			text: text.into()
		}
	}
}
