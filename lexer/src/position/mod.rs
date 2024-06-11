use std::arch::x86_64::__cpuid_count;

#[cfg(test)]
mod test;

/// This is used to track the lines and columns in a file.
/// These line numbers are not the same as the Line Numbers inside a source file.
#[derive(Copy,Clone,Debug)]
pub struct Position {
	line_number: usize,
	column_number: usize,
}

impl Position {
	pub fn new() -> Self {
		return Position {
			line_number: 0,
			column_number: 0,
		}
	}

	pub fn from_lines_and_columns(lines: usize, columns: usize) -> Self {
		return Position {
			line_number: lines,
			column_number: columns,
		}
	}

	pub fn add_columns(self: &mut Self, columns: usize) {
		self.column_number += columns;
	}

	pub fn reset_columns(self: &mut Self) {
		self.column_number = 0;
	}

	pub fn subtract_columns(self: &mut Self, columns: usize) {
		self.column_number -= columns;
	}

	pub fn get_columns(self: &Self) -> usize {
		return self.column_number;
	}

	pub fn add_lines(self: &mut Self, lines: usize) {
		self.line_number += lines;
	}

	pub fn reset_lines(self: &mut Self) {
		self.line_number = 0;
	}

	pub fn subtract_lines(self: &mut Self, lines: usize) {
		self.line_number -= lines;
	}

	pub fn get_lines(self: &Self) -> usize {
		return self.column_number;
	}
}


/// PartialEq is required for testing. Not sure about anything else right now.
#[cfg(test)]
impl PartialEq for Position {
	fn eq(&self, other: &Self) -> bool {
		return self.column_number == other.column_number &&
			self.line_number == other.line_number;
	}
}


/// We can determine full equality for Positions.
#[cfg(test)]
impl Eq for Position {}