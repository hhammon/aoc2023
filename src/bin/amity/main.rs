mod part1;
mod part2;

fn main() {
	let lines = include_str!("input.txt").lines().collect();

	println!("Part 1: {}", part1::sum_calibration_values(&lines));
	println!("Part 2: {}", part2::sum_calibration_values(&lines));
}