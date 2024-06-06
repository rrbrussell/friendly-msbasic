#[cfg(test)]
mod test;

use crate::LexedItems;
use crate::position::Position;

/// This is the parser for the GWBasic version of MSBasic.
#[derive(Debug)]
pub struct Parser<'a> {
	/// Input is a slice of bytes. This might be just a single line or the
	/// entire file depending on what is passed into the parser.
	input: &'a [u8],
	/// This is the current processing location in the input.
	current_character: usize,
	/// This is the human-readable position in the input.
	position_for_humans: Position,
	/// Flag that we have parsed everything in the input slice.
	used_input_buffer: bool,
}

impl<'a> Parser<'a> {
	/// Create a new parser. The parser keeps a readonly reference to the
	/// backing bytes of the input string.
	pub fn new(string: &'a str) -> Self {
		return Parser{
			input: string.as_bytes(),
			current_character: 0,
			position_for_humans: Position::new(),
			used_input_buffer: false,
		}
	}

	/// Replaces the internal buffer with a new one. This does not completely
	/// reset the parsers internal state.
	pub fn replace_buffer(&mut self, buffer: &'a [u8]) {
		self.input = buffer;
		self.current_character = 0;
	}

	/// Get how much of the input buffer has been currently consumed.
	pub fn get_parsed_byte_count(&self) -> usize {
		return self.current_character;
	}

	/// Have we parsed as much of the input buffer as we can use?
	/// self.current_character may not be equal to self.input.len() because we
	/// may need more of the input to finish parsing a particular item. This is
	/// most likely to occur if the input file is being read in chunks instead
	/// of completely into memory at once.
	pub fn used_input_buffer(&self) -> bool {
		return self.used_input_buffer;
	}

	pub fn parse_end_of_line(&mut self) -> Option<LexedItems> {
		let mut lexed_thing: Option<LexedItems> = None;
		// Check if we have two free characters left in the buffer.
		if (self.input.len() - self.current_character) >= 2 {
			let item = &self.input[self.current_character..self.current_character + 2];
			if item == [ 0x0d, 0x0a] {
				lexed_thing = Some(LexedItems::EndOfLine);
				self.current_character += 2;
			}
		}
		return lexed_thing;
	}
}
