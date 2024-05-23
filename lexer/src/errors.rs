pub struct SimpleError{
	message: String,
}

impl SimpleError{
	pub fn new(message: String) -> SimpleError {
		return SimpleError{message };
	}
	pub fn message(self) -> String {
		return self.message;
	}
}