#[cfg(test)]
mod test;

use crate::LexedItems;
use crate::position::Position;

/// The ASCII space character.
pub const SPACE_CHARACTER: u8 = 0x20;
/// The ASCII horizontal tab or plain tab character.
pub const TAB_CHARACTER: u8 = 0x09;

const DIGIT_ZERO: u8 = 0x30;
const DIGIT_ONE: u8 = 0x31;
const DIGIT_TWO: u8 = 0x32;
const DIGIT_THREE: u8 = 0x33;
const DIGIT_FOUR: u8 = 0x34;
const DIGIT_FIVE: u8 = 0x35;
const DIGIT_SIX: u8 = 0x36;
const DIGIT_SEVEN: u8 = 0x37;
const DIGIT_EIGHT: u8 = 0x38;
const DIGIT_NINE: u8 = 0x39;

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

	pub fn get_human_position(&self) -> Position {
		return self.position_for_humans.clone();
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

	/// How many input bytes have we not consumed?
	pub fn get_unparsed_byte_count(&self) -> usize {
		return self.input.len() - self.current_character;
	}

	/// helper function.
	fn move_forward(&mut self, n: usize) {
		self.current_character += n;
		self.position_for_humans.add_columns(n);
	}

	/// Returns None if it could not find an end of line.
	pub fn parse_end_of_line(&mut self) -> Option<LexedItems> {
		let mut lexed_thing: Option<LexedItems> = None;
		// Check if we have two free characters left in the buffer.
		if (self.input.len() - self.current_character) >= 2 {
			let item = &self.input[self.current_character..self.current_character + 2];
			if item == [ 0x0d, 0x0a] {
				lexed_thing = Some(LexedItems::EndOfLine);
				self.current_character += 2;
				self.position_for_humans.add_lines(1);
				self.position_for_humans.reset_columns();
			}
		}
		if (self.input.len() - self.current_character) == 0 {
			self.used_input_buffer = true;
		}
		return lexed_thing;
	}

	/// Gets a run of spaces and tabs.
	pub fn parse_whitespace(&mut self) -> Option<LexedItems> {
		let mut lexed_thing: Option<LexedItems> = None;
		// Check if we have at least one free character left in the buffer.
		if self.get_unparsed_byte_count() >= 1 {
			let ws_start = self.current_character;

			while self.input[self.current_character] == SPACE_CHARACTER ||
				self.input[self.current_character] == TAB_CHARACTER {
				self.current_character += 1;
				if self.current_character >= self.input.len() {
					self.used_input_buffer = true;
					break;
				}
			}
			if self.current_character != ws_start {
				self.position_for_humans.add_columns(self.current_character - ws_start);
				let temp_vector =
					Vec::from(&self.input[ws_start..self.current_character]);
				// We manually checked the contents of these bytes. ASCII characters
				// are valid utf8 by definition.
				unsafe {
					lexed_thing = Some(LexedItems::WhiteSpace(
						String::from_utf8_unchecked(temp_vector)));
				}
			}
		}
		return lexed_thing;
	}

	/// Gets an optional run of spaces and tabs. A None doesn't necessarily mean
	/// a permanent error.
	pub fn parse_optional_whitespace(&mut self) -> Option<LexedItems> {
		let ws = self.parse_whitespace();
		// This works similarly to the ternary operator from C/C++
		return if ws.is_none() {
			None
		} else {
			if let LexedItems::WhiteSpace(data) = ws.unwrap() {
				Some(LexedItems::OptionalWhiteSpace(data))
			} else {
				None
			}
		}
	}

	/// Gets an octal digit.
	pub fn parse_octal_digit(&mut self) -> Option<u8> {
		match self.input.get(self.current_character) {
			None => return None,
			Some(item) => return
				if (*item >= DIGIT_ZERO) && (*item <= DIGIT_SEVEN) {
					self.move_forward(1);
					Some(*item - DIGIT_ZERO)
				} else {
					None
				},
		}
	}
}
