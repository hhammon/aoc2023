Taken from [the website](https://adventofcode.com/2023/day/3):

# --- Day 3: Gear Ratios ---

You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (`.`) do not count as a symbol.)

Here is an example engine schematic:

```
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: `114` (top right) and `58` (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is `4361`.

Of course, the actual engine schematic is much larger. **What is the sum of all of the part numbers in the engine schematic?**

## --- Part Two ---

The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any `*` symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

```
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```

In this schematic, there are two gears. The first is in the top left; it has part numbers `467` and `35`, so its gear ratio is `16345`. The second gear is in the lower right; its gear ratio is `451490`. (The `*` adjacent to `617` is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces `467835`.

**What is the sum of all of the gear ratios in your engine schematic?**

# My Solution

My program correctly gives:

<details>
	<summary>Output</summary>

    Part 1: 550064
    Part 2: 85010461

</details>

## Approach

My solution today was a simple one. I designed some structs to represent the engine schematic in part 1 which I was sure would be useful for part 2. I read the input file as lines and parsed out each number and symbol from each line with some information about location. I then implemented a function to sum up the part numbers. For part 2, all that was needed was to implement a function to sum up the gear ratios. Since the data types were built to represent the schematic, rather than simply to solve part 1, this was a simple task. This is similar to how real-world software is often designed: to be extensible and reusable.

My functions to sum up the part numbers and gear ratios are naive, but they work. I know of more efficient ways to do this, but I'm not sure if it's worth the effort to optimize this code. To find part numbers, I check each number in the schematic and iterate through to symbols of its line and adjacent lines to check for adjacency. To find gear ratios, I iterate through each symbol in the schematic and check if it is a `*` symbol. If it is, I check if it is adjacent to exactly two part numbers, and, if so, I multiply them together and sum them up.

## Analysis

Here, `n` will represent the number of lines in the schematic, `m` will represent the length in each line. It will be assumed that as m grows, the number of symbols and numbers in each line will grow proportionally.

-   Reading the input into my structs runs `O(n * m)` time and `O(n * m)` additional space by simply iterating over the lines and assembling them into a struct character by character.
-   Summing up the part numbers runs in `O(n * m^2)` time and `O(1)` additional space by iterating over the lines, then the numbers in the line, then then symbols in 1 to 3 lines.
-   Summing up the gear ratios also runs in `O(n * m^2)` time and `O(1)` additional space by iterating over the lines, then the symbols in the line, then the numbers in 1 to 3 lines.

It would be pretty easy to convert the two summing functions to run in `O(n * m * log(m))` by using a binary sort for searches since the symbols and numbers are sorted according to their original position. There are also ways that come with separate drawbacks to bring the time complexity to `O(n * m)`.

# Final Thoughts

Today's problem was pleasant to solve overall. I was more focused on experimenting with Rust, doing string manipulation, and working with iterators than on implementing a leet solution, so I didn't spend much time making optimizations like those mentioned above.

I'm not sure how I feel about some of the ways iterators are often used in Rust. I commented briefly on declarative vs imperative programming yesterday. I think that imperative or procedural programming is generally best. This is by no means a criticism of Rust since either method is easily used, but of the method itself. I'll refer to this code from my gear ratio summing function:

```rust
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
```

I think there would be numerous benefits to implementing code like this using simple loops. For example, at the end there's a filter, map, and sum. Prior to these methods being called, a vector containing up to 6 numbers is created. Why get all 6 when they'll be filtered out? While I'm getting the numbers why not simply multiply and sum them? I'm sure it would be easy enough to optimize like that while still using the iterators this way, but it'll start to look more imperative, and as I said, it's probably best done using loops from the start. My problem is that there's a careless way of thinking that seems to be almost inextricably tied to declarative programming and the resulting code suffers for it.

Going forward in these challenges, I'll probably start to revert to my usual imperative and procedural tendencies. This will likely come with better performing code so I won't have to admit to suboptimal performance in my [analysis](#analysis). I'm still happy with my solution today since my goal was to experiment with Rust and I feel like I learned a good amount in doing today's challenge. I'm looking forward to the next one!
