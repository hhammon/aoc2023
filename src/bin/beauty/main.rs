use crate::game::CubeSet;

mod game;
mod part1;
mod part2;

fn main() {
	let input = include_str!("input.txt").lines().collect();
	let games = games_from_input(&input);

	let part1_solution = part1::sum_possible_game_ids(
		&games, &CubeSet {
			red: 12,
			green: 13,
			blue: 14,
		});
	
	let part2_solution = part2::sum_minimum_set_powers(&games);


	println!("Part 1: {}", part1_solution);
	println!("Part 2: {}", part2_solution);

}

fn games_from_input(input: &Vec<&str>) -> Vec<game::Game> {
	input
		.iter()
		.map(|line| game::Game::from_record(line))
		.collect::<Vec<game::Game>>()
}