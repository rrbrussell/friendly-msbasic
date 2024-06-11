pub mod relational_operators;
pub mod errors;

mod reserved_words;
mod basic_tokenizer;

mod parser;
mod position;



#[derive(Debug,PartialEq)]
pub enum LexedItems {
	WhiteSpace(String),
	StringConstant(String),
	LineNumber(usize),
	IntegerVariable(i16),
	IntegerConstant(i16),
	FixedPointConstant(f32),
	FloatingPointConstant(f32),
	HexadecimalConstant(i16),
	OctalConstant(i16),
	Operator(Operators),
	EndOfLine,
	OptionalWhiteSpace(String),
}

#[derive(Debug,PartialEq)]
pub enum Operators {
	Arithmetic(ArithmeticOperators),
	Relational(relational_operators::RelationalOperators),
	Logical(LogicalOperators),
}

#[derive(Debug,PartialEq)]
pub enum ArithmeticOperators {
	Exponentiation,
	Negation,
	Multiplication,
	FloatingPointDivision,
	IntegerDivision,
	Addition,
	Subtraction,
	Modulus,
}

#[derive(Debug,PartialEq)]
pub enum LogicalOperators {
	And,
	Or,
	Not,
	ExclusiveOr,
	Equivalent,
	Implies,
}

#[derive(Debug,PartialEq)]
pub struct LocationAndData {
	pub line_number: usize,
	pub column_number: usize,
	pub data: LexedItems,
}
