use crate::{Error, ImageData};

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

	#[cfg(feature = "image-data")]
	pub(crate) fn get_image(&mut self) -> Result<ImageData<'static>, Error> {
		Err(Error::ContentNotAvailable)
	}

	#[cfg(feature = "image-data")]
	pub(crate) fn set_image(&mut self, image: ImageData) -> Result<(), Error> {
		Ok(())
	}
}
