use super::errors::SimpleError;

#[derive(Debug,PartialEq)]
pub enum RelationalOperators {
	Equality,
	Inequality,
	LessThan,
	GreaterThan,
	LessThanOrEqualTo,
	GreaterThanOrEqualTo,
}

impl TryFrom<String> for RelationalOperators {
	type Error = SimpleError;
	fn try_from(value: String) -> Result<Self, Self::Error> {
		let mut return_value: Option<RelationalOperators> = None;
		if value == "=" {
			return_value = Some(RelationalOperators::Equality);
		}
		if value == "<>" {
			return_value = Some(RelationalOperators::Inequality);
		}
		if value == "<" {
			return_value = Some(RelationalOperators::LessThan);
		}
		if value == ">" {
			return_value = Some(RelationalOperators::GreaterThan);
		}
		if value == "<=" {
			return_value = Some(RelationalOperators::LessThanOrEqualTo);
		}
		if value == ">=" {
			return_value = Some(RelationalOperators::GreaterThanOrEqualTo);
		}
		// This is equivalent to the C++ ternary operator.
		return if return_value.is_some() {
			Ok(return_value.unwrap())
		} else {
			let msg: SimpleError =
				SimpleError::new(format!("{} is not a valid relational operator", value));
			Err(msg)
		}
	}
}