use std::str::Chars;

pub fn sum_calibration_values(lines: &Vec<&str>) -> u32 {
	lines.iter().map(|line| {
		let first_digit = find_digit(line.chars(), false);
		let last_digit = match first_digit {
			Some(_) => find_digit(line.chars().rev(), true),
			None => None,
		};
		
		first_digit.unwrap_or(0) * 10 + last_digit.unwrap_or(0)
	}).sum()
}

fn find_digit(chars: impl Iterator<Item = char>, reversed: bool) -> Option<u32> {
	let mut words_checking: [(u32, Chars); 5] = [
		(0, "".chars()),
		(0, "".chars()),
		(0, "".chars()),
		(0, "".chars()),
		(0, "".chars()),
	];
	let mut words_count: usize = 0;

	for c in chars {
		let c = c.to_lowercase().next().unwrap();

		let mut index: usize = 0;
		while index < words_count {
			let (digit, word_chars) = &mut words_checking[index];
			if let Some(wc) = word_chars.next() {
				if wc != c {
					words_checking.swap(index, words_count - 1);
					words_count -= 1;
				} else {
					index += 1;
				}
			} else {
				return Some(*digit);
			}
		}

		if !reversed {
			match c {
				'z' => {
					words_checking[words_count] = (0, "ero".chars());
					words_count += 1;
				},
				'o' => {
					words_checking[words_count] = (1, "ne".chars());
					words_count += 1;
				},
				't' => {
					words_checking[words_count] = (2, "wo".chars());
					words_checking[words_count + 1] = (3, "hree".chars());
					words_count += 2;
				},
				'f' => {
					words_checking[words_count] = (4, "our".chars());
					words_checking[words_count + 1] = (5, "ive".chars());
					words_count += 2;
				},
				's' => {
					words_checking[words_count] = (6, "ix".chars());
					words_checking[words_count + 1] = (7, "even".chars());
					words_count += 2;
				},
				'e' => {
					words_checking[words_count] = (8, "ight".chars());
					words_count += 1;
				},
				'n' => {
					words_checking[words_count] = (9, "ine".chars());
					words_count += 1;
				},
				_ => {}
			}
		} else {
			match c {
				'o' => {
					words_checking[words_count] = (0, "rez".chars());
					words_checking[words_count + 1] = (2, "wt".chars());
					words_count += 2;
				},
				'e' => {
					words_checking[words_count] = (1, "no".chars());
					words_checking[words_count + 1] = (3, "erht".chars());
					words_checking[words_count + 2] = (5, "vif".chars());
					words_checking[words_count + 3] = (9, "nin".chars());
					words_count += 4;
				},
				'r' => {
					words_checking[words_count] = (4, "uof".chars());
					words_count += 1;
				},
				'x' => {
					words_checking[words_count] = (6, "is".chars());
					words_count += 1;
				},
				'n' => {
					words_checking[words_count] = (7, "eves".chars());
					words_count += 1;
				},
				't' => {
					words_checking[words_count] = (8, "hgie".chars());
					words_count += 1;
				},
				_ => {}
			}
		}

		if c.is_digit(10) {
			return Some(c.to_digit(10).unwrap());
		}
	}

	for (
		digit,
		word_chars
	) in words_checking.iter_mut().take(words_count) {
		if word_chars.next().is_none() {
			return Some(*digit);
		}
	}

	None
}