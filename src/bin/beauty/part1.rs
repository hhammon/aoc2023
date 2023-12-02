use crate::game::{Game, CubeSet};

pub fn sum_possible_game_ids(
	games: &Vec<Game>,
	max_set: &CubeSet
) -> u32 {
	games
		.iter()
		.filter(|game| game.is_possible(
			max_set.red,
			max_set.green,
			max_set.blue,
		))
		.map(|game| game.id)
		.sum()
}