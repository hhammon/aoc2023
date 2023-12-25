use crate::almanac::{Almanac, AlmanacParseError};

mod almanac;

fn main() {
	let almanac = Almanac::parse_str(include_str!("input.txt"));

	if let Err(error) = almanac {
		return match error {
			AlmanacParseError::MissingSeeds => eprintln!("Missing seeds"),
			AlmanacParseError::InvalidSeed => eprintln!("Invalid seed"),
			AlmanacParseError::OddNumberOfSeeds => eprintln!("Odd number of seeds"),
			AlmanacParseError::InvalidMapName => eprintln!("Invalid map name"),
			AlmanacParseError::InvalidRange => eprintln!("Invalid range"),
		};
	}

	let almanac = almanac.unwrap();

	match almanac.lowest_location() {
		Some(location) => println!("Part 1: {}", location),
		None => eprintln!("Unable to map seeds to locations"),
	}
}