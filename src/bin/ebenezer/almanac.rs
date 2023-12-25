use std::{collections::HashMap, num::ParseIntError};

struct RangeMap {
	source_start: u64,
	destination_start: u64,
	length: u64,
}

struct Range {
	start: u64,
	length: u64,
} 

struct SourceToDestinationMap {
	destination: String,
	ranges: Vec<RangeMap>,
}

impl SourceToDestinationMap {
	pub fn map(
		&self,
		source_values: &Vec<u64>,
	) -> Vec<u64> {
		source_values.iter().map(move |value| {
			for range in &self.ranges {
				if range.source_start <= *value && *value < range.source_start + range.length {
					return range.destination_start + *value - range.source_start;
				}
			}

			*value
		}).collect()
	} 

	pub fn map_ranges(
		&self,
		source_ranges: &Vec<Range>,
	) -> Vec<Range> {
		let mut ranges: Vec<Range> = Vec::new();
		source_ranges.iter().for_each(|source_range| {
			for range in &self.ranges {
				if range.source_start <= source_range.start && source_range.start < range.source_start + range.length {
					let destination_start = range.destination_start + source_range.start - range.source_start;
					let destination_end = destination_start + source_range.length;

					ranges.push(Range {
						start: destination_start,
						length: destination_end - destination_start,
					});
				}
			}
		});

		ranges
	}
}

pub struct Almanac {
	seeds: Vec<u64>,
	seed_ranges: Vec<Range>,
	maps: HashMap<String, SourceToDestinationMap>,
}

#[derive(Debug)]
pub enum AlmanacParseError {
	MissingSeeds,
	InvalidSeed,
	OddNumberOfSeeds,
	InvalidMapName,
	InvalidRange,
}

impl Almanac {
	pub fn parse_str(input: &str) -> Result<Self, AlmanacParseError> {
		let mut lines = input.lines();

		let seeds_line = lines
			.next()
			.ok_or(AlmanacParseError::MissingSeeds)?
			.trim();

		if !seeds_line.starts_with("seeds: ") {
			return Err(AlmanacParseError::MissingSeeds);
		}

		let seeds = seeds_line
			.split_whitespace()
			.skip(1)
			.map(|seed| seed.parse())
			.collect::<Result<Vec<u64>, ParseIntError>>()
			.map_err(|_| AlmanacParseError::InvalidSeed)?;

		if seeds.len() % 2 != 0 {
			return Err(AlmanacParseError::OddNumberOfSeeds);
		}

		let mut seed_ranges = Vec::with_capacity(seeds.len() / 2);
		let mut seed_range = Range {
			start: 0,
			length: 0,
		};

		for (i, seed) in seeds.iter().enumerate() {
			if i % 2 == 0 {
				seed_range.start = *seed;
			} else {
				seed_range.length = *seed;

				seed_ranges.push(seed_range);
				seed_range = Range {
					start: 0,
					length: 0,
				};
			}
		}

		let mut maps: HashMap<String, SourceToDestinationMap> = HashMap::new();

		let mut working_on_map = false;
		let mut source: &str = "";
		let mut destination: &str = "";
		let mut ranges: Vec<RangeMap> = Vec::new();

		for line in lines {
			let line = line.trim();

			if line.is_empty() {
				continue;
			}

			if line.ends_with(" map:") {
				if working_on_map {
					maps.insert(source.to_string(), SourceToDestinationMap {
						destination: destination.to_string(),
						ranges,
					});
				}

				working_on_map = true;

				let mut map_name_parts = line
					.split_whitespace()
					.next().unwrap()
					.split('-');
				
				source = map_name_parts
					.next()
					.ok_or(AlmanacParseError::InvalidMapName)?
					.trim();

				map_name_parts
					.next()
					.ok_or(AlmanacParseError::InvalidMapName)?;

				destination = map_name_parts
					.next()
					.ok_or(AlmanacParseError::InvalidMapName)?
					.trim();

				ranges = Vec::new();
			} else if working_on_map {
				let mut range_parts = line.split_whitespace();
				
				ranges.push(
					RangeMap {
						destination_start: range_parts
							.next()
							.ok_or(AlmanacParseError::InvalidRange)?
							.parse()
							.map_err(|_| AlmanacParseError::InvalidRange)?,

						source_start: range_parts
							.next()
							.ok_or(AlmanacParseError::InvalidRange)?
							.parse()
							.map_err(|_| AlmanacParseError::InvalidRange)?,
							
						length: range_parts
							.next()
							.ok_or(AlmanacParseError::InvalidRange)?
							.parse()
							.map_err(|_| AlmanacParseError::InvalidRange)?,
					}
				);
			}
		}

		if working_on_map {
			maps.insert(source.to_string(), SourceToDestinationMap {
				destination: destination.to_string(),
				ranges,
			});
		}

		Ok(Self {
			seeds,
			seed_ranges,
			maps,
		})
	}

	fn destination_values(
		&self,
		source: &str,
		destination: &str,
		source_values: &Vec<u64>,
	) -> Option<Vec<u64>> {
		let mut map = self.maps.get(source)?;
		let mut values = map.map(source_values);

		while map.destination != destination {
			map = self.maps.get(&map.destination)?;

			values = map.map(&values);
		}

		Some(values)
	}

	fn destination_ranges(
		&self,
		source: &str,
		destination: &str,
		source_ranges: &Vec<Range>,
	) -> Option<Vec<Range>> {
		let mut map = self.maps.get(source)?;

		None
	}

	pub fn seed_locations(&self) -> Option<Vec<u64>> {
		self.destination_values("seed", "location", &self.seeds)
	}

	pub fn lowest_location(&self) -> Option<u64> {
		self.seed_locations()?.into_iter().min()
	}
}