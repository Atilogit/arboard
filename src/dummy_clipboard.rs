use crate::Error;

pub struct DummyClipboardContext;

impl DummyClipboardContext {
	pub(crate) fn new() -> Result<Self, Error> {
		Ok(DummyClipboardContext)
	}

	pub(crate) fn get_text(&mut self) -> Result<String, Error> {
		Ok("".to_owned())
	}

	pub(crate) fn set_text(&mut self, data: String) -> Result<(), Error> {
		Ok(())
	}
}
