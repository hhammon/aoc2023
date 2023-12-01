
pub fn sum_calibration_values(lines: &Vec<&str>) -> u32 {
	lines.iter().map(|line| {
		let first_digit = find_digit(line.chars());
		let last_digit = match first_digit {
			Some(_) => find_digit(line.chars().rev()),
			None => None,
		};
		
		first_digit.unwrap_or(0) * 10 + last_digit.unwrap_or(0)
	}).sum()
}

fn find_digit(chars: impl Iterator<Item = char>) -> Option<u32> {
	for c in chars {
		if c.is_digit(10) {
			return Some(c.to_digit(10).unwrap());
		}
	}

	None
}