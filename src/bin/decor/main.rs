use scratchcard::{Scratchcard, ScratchcardParseError, ScratchcardPile};

mod scratchcard;

fn main() {
	let cards: Result<Vec<Scratchcard>, ScratchcardParseError> = 
		include_str!("input.txt")
		.lines()
		.map(|line| Scratchcard::parse_line(line))
		.collect();

	if let Err(kind) = cards {
		return match kind {
			ScratchcardParseError::MalformedCard => {
				eprintln!("Malformed card")
			},
			ScratchcardParseError::InvalidId => {
				eprintln!("Invalid ID")
			},
			ScratchcardParseError::InvalidWinningNumbers => {
				eprintln!("Invalid winning numbers")
			},
			ScratchcardParseError::InvalidNumbersYouHave => {
				eprintln!("Invalid card numbers")
			},
		}
	}

	let cards = cards.unwrap();

	println!("Part 1: {}", cards.value());
	println!("Part 2: {}", cards.copies());
}