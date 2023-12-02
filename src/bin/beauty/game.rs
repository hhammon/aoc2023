use std::cmp::max;

pub struct CubeSet {
	pub red: u32,
	pub green: u32,
	pub blue: u32,
}

impl CubeSet {
	pub fn from_record(record: &str) -> Self {
		let mut red: u32 = 0;
		let mut green: u32 = 0;
		let mut blue: u32 = 0;

		for cube_record in record.split(',') {
			let mut parts = cube_record
				.trim()
				.split_whitespace();

			let value = parts
				.next()
				.expect("Cube record must have a value")
				.parse::<u32>()
				.expect("Cube record value must be a number");
			
			let color = parts
				.next()
				.expect("Cube record must have a color");

			match color {
				"red" => red += value,
				"green" => green += value,
				"blue" => blue += value,
				_ => panic!("Unknown color: {}", color),
			}
		}

		Self {
			red,
			green,
			blue,
		}
	}

	pub fn is_possible(
		&self,
		max_red: u32,
		max_green: u32,
		max_blue: u32,
	) -> bool {
		self.red <= max_red && 
		self.green <= max_green &&
		self.blue <= max_blue
	}

	pub fn power(&self) -> u32 {
		self.red * self.green * self.blue
	}
}

pub struct Game {
	pub id: u32,
	pub cube_sets: Vec<CubeSet>,
}

impl Game {
	pub fn from_record(record: &str) -> Self {
		let mut parts = record.split(':');

		let id = parts
			.next()
			.expect("Game record must have an ID")
			.chars()
			.filter(|c| c.is_digit(10))
			.collect::<String>()
			.parse::<u32>()
			.unwrap();

		let cube_sets = parts
			.next()
			.expect("Game record must have cube sets")
			.split(';')
			.map(|cube_set_record| CubeSet::from_record(cube_set_record))
			.collect::<Vec<CubeSet>>();
		

		Self {
			id,
			cube_sets,
		}
	}

	pub fn is_possible(
		&self,
		max_red: u32,
		max_green: u32,
		max_blue: u32,
	) -> bool {
		self.cube_sets
			.iter()
			.all(|cube_set| cube_set.is_possible(
				max_red,
				max_green,
				max_blue
			))
	}

	pub fn minimum_set(&self) -> CubeSet {
		let mut red: u32 = 0;
		let mut green: u32 = 0;
		let mut blue: u32 = 0;

		for cube_set in &self.cube_sets {
			red = max(red, cube_set.red);
			green = max(green, cube_set.green);
			blue = max(blue, cube_set.blue);
		}

		CubeSet {
			red,
			green,
			blue,
		}
	}
}