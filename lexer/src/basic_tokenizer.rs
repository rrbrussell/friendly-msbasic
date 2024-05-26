use std::iter::Peekable;
/// The most basic tokenizer tool.

use std::str::{CharIndices, Lines};
struct BasicTokenizer<'a> {
	input_iterator: Lines<'a>,
	current_line_iterator: Option<CharIndices<'a>>,
	peeked_item: Option<(usize, char)>,
	position: Position,
	completed: bool,
	temp_token: Option<BasicToken>,
}

struct Position {
	pub line: usize,
	pub column: usize,
}
impl BasicTokenizer<'_> {
	fn new(input_string: &str) -> BasicTokenizer {
		BasicTokenizer{
			input_iterator: input_string.lines(),
			current_line_iterator: None,
			peeked_item: None,
			position: Position{ line: 0, column: 0},
			completed: false,
			temp_token: None,
		}
	}
}

impl Iterator for BasicTokenizer<'_> {
	type Item = BasicToken;
	fn next(&mut self) -> Option<Self::Item> {
		if self.completed {
			return None;
		}
		let mut st: String = String::with_capacity(255);

		match self.peeked_item {
			Some((index, character)) => {
				if character.is_alphabetic() {
					if character.is_numeric() {
						st.push(character);
					} else {
						st.push(character);
					}
				} else {

				}
			}
			None => {

			}
		}
		todo!();
		return None;
	}
}

pub enum BasicToken {
	Whitespace(String),
	AlphaNumericString(String),
	NumericString(String),
	EqualSign,
	MinusSign,
	Asterisk,
	ForwardSlash,
	Caret,
	LeftParenthesis,
	RightParenthesis,
	PercentSign,
	NumberSign,
	DollarSign,
	ExclamationPoint,
	LeftBracket,
	RightBracket,
	Comma,
	DoubleQuotationMark,
	Period,
	SingleQuotationMark,
	Semicolon,
	Colon,
	Ampersand,
	QuestionMark,
	LessThanSymbol,
	GreaterThanSymbol,
	BackwardSlash,
	AtSign,
	Underscore,
}

