use super::*;
use crate::position::Position;

#[test]
fn test_parse_end_of_line() {
	let mut parser :Parser = Parser::new("\r\n\n");
	assert_ne!("\r\n", "\n"); // assert that \n doesn't include \r.
	// This is mostly a sanity check.
	let mut test_result :Option<LexedItems> = parser.parse_end_of_line();
	assert!(test_result.is_some());
	assert_eq!(test_result.unwrap(), LexedItems::EndOfLine);
	test_result = parser.parse_end_of_line();
	assert!(test_result.is_none());
}

#[test]
fn test_parse_whitespace() {
	let mut parser :Parser = Parser::new(" \tabcd");
	let mut test_result :Option<LexedItems> = parser.parse_whitespace();
	let mut position = parser.get_human_position();
	assert!(test_result.is_some());
	assert_eq!(position, Position::from_lines_and_columns(0,2));
	test_result = parser.parse_whitespace();
	position = parser.get_human_position();
	assert!(test_result.is_none());
	assert_eq!(position, Position::from_lines_and_columns(0, 2));
}

#[test]
fn test_parse_octal_digit() {
	let mut parser: Parser = Parser::new("012345678");
	for x in 0..8u8 {
		assert_eq!(parser.get_human_position().get_columns(), x as usize);
		let test_result: Option<char> = parser.parse_octal_digit();
		let x_char: char = char::from(x + DIGIT_ZERO);
		assert_eq!(test_result, Some(x_char));
		assert_eq!(parser.get_human_position().get_columns(), (x+1) as usize);
	}
	assert_eq!(parser.get_human_position().get_columns(), 8);
	assert_eq!(parser.parse_octal_digit(), None);
	assert_eq!(parser.get_human_position().get_columns(), 8);
}