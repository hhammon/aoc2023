
pub struct EngineSchematic {
	lines: Vec<SchematicLine>,
}

impl EngineSchematic {
	pub fn new(schema: &str) -> Self {
		Self {
			lines: schema
			.lines()
			.map(|line| SchematicLine::new(line))
			.collect(),
		}
	}

	pub fn sum_part_numbers(&self) -> u32 {
		//Part numbers are adjacent to symbols, even diagonally
		self.lines
		.iter()
		.enumerate()
		.map(|(line_index, line)|
			line.numbers
			.iter()
			.filter(|number| {
				line.symbols
				.iter()
				.any(|symbol|
					symbol.index + 1 == number.start || 
					symbol.index == number.end + 1
				)
				||
				(line_index > 0)
				.then(|| &self.lines[line_index - 1].symbols)
				.into_iter()
				.flatten()
				.chain(
					(line_index < self.lines.len() - 1)
					.then(|| &self.lines[line_index + 1].symbols)
					.into_iter()
					.flatten()
				)
				.any(|symbol: &SchematicSymbol| -> bool {
					symbol.index + 1 >= number.start && 
					symbol.index <= number.end + 1
				})
			})
			.map(|number| number.value)
			.sum::<u32>()
		)
		.sum()
	}

	pub fn sum_gear_ratios(&self) -> u32 {
		//A gear is a * symbol adjacent to exactly two numbers
		//The gear ratio is the product of the two numbers
		self.lines
		.iter()
		.enumerate()
		.map(|(line_index, line)|
			line.symbols
			.iter()
			.filter(|symbol| symbol.value == '*')
			.map(|symbol| {
				let mut adjacent_numbers: Vec<&SchematicNumber> = Vec::with_capacity(6);

				adjacent_numbers.extend(
					line.numbers
					.iter()
					.filter(|number| 
						number.start == symbol.index + 1 || 
						number.end == symbol.index - 1
					)
				);

				let previous_line =
				(line_index > 0)
				.then(|| &self.lines[line_index - 1].numbers)
				.into_iter()
				.flatten();

				let next_line =
				(line_index < self.lines.len() - 1)
				.then(|| &self.lines[line_index + 1].numbers)
				.into_iter()
				.flatten();

				adjacent_numbers.extend(
					previous_line
					.chain(next_line)
					.filter(|number| 
						number.start <= symbol.index + 1 && 
						number.end >= symbol.index - 1
					)
				);

				adjacent_numbers
			})
			.filter(|adjacent_numbers| adjacent_numbers.len() == 2)
			.map(|adjacent_numbers| 
				adjacent_numbers
				.iter()
				.map(|number| number.value)
				.product::<u32>()
			)
			.sum::<u32>()
		)
		.sum()
	}
}

struct SchematicLine {
	numbers: Vec<SchematicNumber>,
	symbols: Vec<SchematicSymbol>,
}

impl SchematicLine {
	fn new(line: &str) -> Self {
		let mut numbers: Vec<SchematicNumber> = Vec::new();
		let mut symbols: Vec<SchematicSymbol> = Vec::new();

		let mut processing_number = false;
		let mut number: u32 = 0;
		let mut number_start = 0;

		for (index, character) in line.chars().enumerate() {
			match character {
				'.' => {
					if processing_number {
						processing_number = false;
						numbers.push(SchematicNumber {
							value: number,
							start: number_start,
							end: index - 1,
						});
					}
				},
				'0'..='9' => {
					if processing_number {
						number *= 10;
						number += character.to_digit(10).unwrap();
					} else {
						processing_number = true;
						number_start = index;
						number = character.to_digit(10).unwrap();
					}
				},
				_ => {
					if processing_number {
						processing_number = false;
						numbers.push(SchematicNumber {
							value: number,
							start: number_start,
							end: index - 1,
						});
					}

					symbols.push(SchematicSymbol {
						value: character,
						index,
					});
				}
			}
		}

		if processing_number {
			numbers.push(SchematicNumber {
				value: number,
				start: number_start,
				end: line.len() - 1,
			});
		}

		Self {
			numbers,
			symbols,
		}
	}
}

struct SchematicNumber {
	value: u32,
	start: usize,
	end: usize,
}

struct SchematicSymbol {
	value: char,
	index: usize,
}