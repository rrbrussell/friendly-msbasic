use super::*;

#[test]
fn test_partial_eq() {
	assert_eq!(Position{ line_number:0, column_number:0 },
			   Position{ line_number:0, column_number:0 });
	assert_ne!(Position{ line_number:1, column_number:0 },
			   Position{ line_number:0, column_number:1 });
}
