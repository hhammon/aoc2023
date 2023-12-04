use std::{num::ParseIntError, collections::HashSet, cmp::min};

pub struct Scratchcard {
	pub id: u32,
	pub winning_numbers: Vec<u32>,
	pub numbers_you_have: Vec<u32>,
}

#[derive(Debug)]
pub enum ScratchcardParseError {
	MalformedCard,
	InvalidId,
	InvalidWinningNumbers,
	InvalidNumbersYouHave,
}

impl Scratchcard {
	pub fn parse_line(line: &str) -> Result<Self, ScratchcardParseError> {
		let mut iter = line.split(':');

		let id_part = iter
			.next()
			.ok_or(ScratchcardParseError::MalformedCard)?;

		let numbers_part = iter
			.next()
			.ok_or(ScratchcardParseError::MalformedCard)?;
		
		let id = id_part
			.chars()
			.filter(|c| c.is_digit(10))
			.collect::<String>()
			.parse::<u32>()
			.map_err(|_| ScratchcardParseError::InvalidId)?;

		let mut numbers_iter = numbers_part.split('|');

		let winning_numbers = str_to_u32_vec(
			numbers_iter
				.next()
				.ok_or(ScratchcardParseError::MalformedCard)?
		)
		.map_err(|_| ScratchcardParseError::InvalidWinningNumbers)?;

		let numbers_you_have = str_to_u32_vec(
			numbers_iter
				.next()
				.ok_or(ScratchcardParseError::MalformedCard)?
		)
		.map_err(|_| ScratchcardParseError::InvalidNumbersYouHave)?;

		Ok(Self {
			id,
			winning_numbers,
			numbers_you_have,
		})
	}

	pub fn match_count(&self) -> usize {
		let winning_numbers_set: HashSet<u32> = self.winning_numbers
			.iter()
			.cloned()
			.collect();

		self.numbers_you_have
			.iter()
			.filter(|n| winning_numbers_set.contains(n))
			.count()
	}

	pub fn value(&self) -> u32 {
		(1 << self.match_count()) >> 1
	}
}

pub trait ScratchcardPile {
	fn value(&self) -> u32;
	fn copies(&self) -> u32;
}

impl ScratchcardPile for Vec<Scratchcard> {
	fn value(&self) -> u32 {
		self
			.iter()
			.map(|card| card.value())
			.sum()
	}

	fn copies(&self) -> u32 {
		let mut cards_and_copies: Vec<(&Scratchcard, u32)> = self
			.iter()
			.map(|card| (card, 1))
			.collect::<Vec<_>>();

		let mut count: u32 = 0;
		for index in 0..cards_and_copies.len() {
			let (card, copies) = cards_and_copies[index];

			count += copies;

			let start = index + 1;
			let end = min(start + card.match_count(), self.len());
			for next_card_index in start..end {
				cards_and_copies[next_card_index].1 += copies;
			}
		}

		count
	}
}

fn str_to_u32_vec(s: &str) -> Result<Vec<u32>, ParseIntError> {
	s.split_whitespace()
		.filter(|s| 
			s.chars().all(|c| c.is_digit(10))
		)
		.map(|s| 
			s.parse::<u32>()
		)
		.collect()
}