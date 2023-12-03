mod schematic;

fn main() {
	let input = include_str!("input.txt");
	let engine_schema = schematic::EngineSchematic::new(input);

	println!("Part 1: {}", engine_schema.sum_part_numbers());
	println!("Part 2: {}", engine_schema.sum_gear_ratios());
}