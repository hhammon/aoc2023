use crate::game::Game;

pub fn sum_minimum_set_powers(games: &Vec<Game>) -> u32 {
	games
		.iter()
		.map(|game| game.minimum_set().power())
		.sum()
}