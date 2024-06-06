use super::*;

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
	let mut parser :Parser = Parser::new(" \t");
	let mut test_result :Option<LexedItems> = parser.parse_whitespace();
	assert!(test_result.is_some());
}