Taken from [the website](https://adventofcode.com/2023/day/1):

# --- Day 1: Trebuchet?! ---

Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

```
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```

In this example, the calibration values of these four lines are `12`, `38`, `15`, and `77`. Adding these together produces `142`.

Consider your entire calibration document. **What is the sum of all of the calibration values?**

## --- Part Two ---

Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

```
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
```

In this example, the calibration values are `29`, `83`, `13`, `24`, `42`, `14`, and `76`. Adding these together produces `281`.

**What is the sum of all of the calibration values?**

# My Solution

My code correctly yields:

```
Part 1: 54916
Part 2: 54728
```

Part 1 was pretty straightforward. I just needed to parse the input and sum the first and last digits of each line. Finding the digits was simple. I just iterated through the characters in each line, first forward, then in reverse, until I found a digit. Then, I created a number from each digit and summed them all.

Part 2 was interesting with several good ways to tackle it. I chose to take an involved approach by iterating through the characters exactly as I did in Part 1 while searching for the text representations of each digit. There was, of course, a simpler way through RegEx or something similar, but that carries the significant disadvantage of using RegEx. It also didn't seem to be in the spirit of the problem. I wanted to find the digits myself. I'm happy with my solution except for the verbosity of the code resulting from that decision. Nearly half of the code file is this curiosity:

```rust
if !reversed {
	match c {
		'z' => {
			words_checking[words_count] = (0, "ero".chars());
			words_count += 1;
		},
		'o' => {
			words_checking[words_count] = (1, "ne".chars());
			words_count += 1;
		},
		't' => {
			words_checking[words_count] = (2, "wo".chars());
			words_checking[words_count + 1] = (3, "hree".chars());
			words_count += 2;
		},
		'f' => {
			words_checking[words_count] = (4, "our".chars());
			words_checking[words_count + 1] = (5, "ive".chars());
			words_count += 2;
		},
		's' => {
			words_checking[words_count] = (6, "ix".chars());
			words_checking[words_count + 1] = (7, "even".chars());
			words_count += 2;
		},
		'e' => {
			words_checking[words_count] = (8, "ight".chars());
			words_count += 1;
		},
		'n' => {
			words_checking[words_count] = (9, "ine".chars());
			words_count += 1;
		},
		_ => {}
	}
} else {
	match c {
		'o' => {
			words_checking[words_count] = (0, "rez".chars());
			words_checking[words_count + 1] = (2, "wt".chars());
			words_count += 2;
		},
		'e' => {
			words_checking[words_count] = (1, "no".chars());
			words_checking[words_count + 1] = (3, "erht".chars());
			words_checking[words_count + 2] = (5, "vif".chars());
			words_checking[words_count + 3] = (9, "nin".chars());
			words_count += 4;
		},
		'r' => {
			words_checking[words_count] = (4, "uof".chars());
			words_count += 1;
		},
		'x' => {
			words_checking[words_count] = (6, "is".chars());
			words_count += 1;
		},
		'n' => {
			words_checking[words_count] = (7, "eves".chars());
			words_count += 1;
		},
		't' => {
			words_checking[words_count] = (8, "hgie".chars());
			words_count += 1;
		},
		_ => {}
	}
}

```

Here, `words_checking` and `words_count` are defined as:

```rust
let mut words_checking: [(u32, Chars); 5];
let mut words_count: usize;
```

The tuple in the definition of `words_checking` holds a `u32`, which is the digit associated with the word being matched, and a `Chars`, which is an iterator of the word itself. As I iterate through the characters in each line, I compare with the next character in each word being checked. I was able to determine that a length of 5 was necessary and sufficient for the array. This allows me to avoid using a vector, linked list, or some similar data type which would be more expensive in some way. If a word doesn't match, I simply run the following to remove it:

```rust
words_checking.swap(index, words_count - 1);
words_count -= 1;
```

This gives a level of performance I can be satisfied with.

## Analysis

With `n` representing the number of lines and `m` representing the average line length, my solution to both parts is at worst `O(n * m)` and at best `O(n)` in time complexity, and `O(1)` in space complexity. The worst case is an input where no line contains a digit and every character must be checked. The best case is an input where every line contains a digit at the beginning and end and 2 characters must be checked per line.

# Final Thoughts

This challenge was a relaxed and enjoyable introduction to Advent of Code, and a good warm-up to Rust, a language I'm enjoying but am still relatively inexperienced with. I'm excited for the next one!
