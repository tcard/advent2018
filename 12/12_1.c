// --- Day 12: Subterranean Sustainability ---

// The year 518 is significantly more underground than your history books
// implied. Either that, or you've arrived in a vast cavern network under the
// North Pole.

// After exploring a little, you discover a long tunnel that contains a row of
// small pots as far as you can see to your left and right. A few of them
// contain plants - someone is trying to grow things in these geothermally-
// heated caves.

// The pots are numbered, with 0 in front of you. To the left, the pots are
// numbered -1, -2, -3, and so on; to the right, 1, 2, 3.... Your puzzle input
// contains a list of pots from 0 to the right and whether they do (#) or do not
// (.) currently contain a plant, the initial state. (No other pots currently
// contain plants.) For example, an initial state of #..##.... indicates that
// pots 0, 3, and 4 currently contain plants.

// Your puzzle input also contains some notes you find on a nearby table:
// someone has been trying to figure out how these plants spread to nearby pots.
// Based on the notes, for each generation of plants, a given pot has or does
// not have a plant based on whether that pot (and the two pots on either side
// of it) had a plant in the last generation. These are written as LLCRR => N,
// where L are pots to the left, C is the current pot being considered, R are
// the pots to the right, and N is whether the current pot will have a plant in
// the next generation. For example:

// A note like ..#.. => . means that a pot that contains a plant but with no
// plants within two pots of it will not have a plant in it during the next
// generation.
// A note like ##.## => . means that an empty pot with two plants on each side
// of it will remain empty in the next generation.
// A note like .##.# => # means that a pot has a plant in a given generation if,
// in the previous generation, there were plants in that pot, the one
// immediately to the left, and the one two pots to the right, but not in the
// ones immediately to the right and two to the left.

// It's not clear what these plants are for, but you're sure it's important, so
// you'd like to make sure the current configuration of plants is sustainable by
// determining what will happen after 20 generations.

// For example, given the following input:

// initial state: #..#.#..##......###...###

// ...## => #
// ..#.. => #
// .#... => #
// .#.#. => #
// .#.## => #
// .##.. => #
// .#### => #
// #.#.# => #
// #.### => #
// ##.#. => #
// ##.## => #
// ###.. => #
// ###.# => #
// ####. => #

// For brevity, in this example, only the combinations which do produce a plant
// are listed. (Your input includes all possible combinations.) Then, the next
// 20 generations will look like this:

//                  1         2         3     
//        0         0         0         0     
//  0: ...#..#.#..##......###...###...........
//  1: ...#...#....#.....#..#..#..#...........
//  2: ...##..##...##....#..#..#..##..........
//  3: ..#.#...#..#.#....#..#..#...#..........
//  4: ...#.#..#...#.#...#..#..##..##.........
//  5: ....#...##...#.#..#..#...#...#.........
//  6: ....##.#.#....#...#..##..##..##........
//  7: ...#..###.#...##..#...#...#...#........
//  8: ...#....##.#.#.#..##..##..##..##.......
//  9: ...##..#..#####....#...#...#...#.......
// 10: ..#.#..#...#.##....##..##..##..##......
// 11: ...#...##...#.#...#.#...#...#...#......
// 12: ...##.#.#....#.#...#.#..##..##..##.....
// 13: ..#..###.#....#.#...#....#...#...#.....
// 14: ..#....##.#....#.#..##...##..##..##....
// 15: ..##..#..#.#....#....#..#.#...#...#....
// 16: .#.#..#...#.#...##...#...#.#..##..##...
// 17: ..#...##...#.#.#.#...##...#....#...#...
// 18: ..##.#.#....#####.#.#.#...##...##..##..
// 19: .#..###.#..#.#.#######.#.#.#..#.#...#..
// 20: .#....##....#####...#######....#.#..##.

// The generation is shown along the left, where 0 is the initial state. The pot
// numbers are shown along the top, where 0 labels the center pot, negative-
// numbered pots extend to the left, and positive pots extend toward the right.
// Remember, the initial state begins at pot 0, which is not the leftmost pot
// used in this example.

// After one generation, only seven plants remain. The one in pot 0 matched the
// rule looking for ..#.., the one in pot 4 matched the rule looking for .#.#.,
// pot 9 matched .##.., and so on.

// In this example, after 20 generations, the pots shown as # contain plants,
// the furthest left of which is pot -2, and the furthest right of which is pot
// 34. Adding up all the numbers of plant-containing pots after the 20th
// generation produces 325.

// After 20 generations, what is the sum of the numbers of all pots which
// contain a plant?


#include <stdio.h>
#include <string.h>

#define NUM_GENERATIONS 20

#define RULE_PATTERN_SIZE 5
#define RULE_SIZE (RULE_PATTERN_SIZE + 1) // Last is outcome
#define RULE_OUTCOME RULE_PATTERN_SIZE
#define NUM_RULES 32 // 2 ^ RULE_PATTERN_SIZE

#define INITIAL_STATE_SIZE 100
#define GROWTH_LEEWAY ((RULE_PATTERN_SIZE - 2) * NUM_GENERATIONS) // Can only grow this much either way
#define MAX_STATE_SIZE (GROWTH_LEEWAY + INITIAL_STATE_SIZE + GROWTH_LEEWAY)
#define STATE_SIZE (MAX_STATE_SIZE + 1) // Last is null byte

#define RULE_LINE_LEN sizeof("..... => .\n")

void init_state(char state[STATE_SIZE]) {
	for (int i = 0; i < GROWTH_LEEWAY; i++) {
		state[i] = '.';
	}
	for (int i = GROWTH_LEEWAY + INITIAL_STATE_SIZE; i < MAX_STATE_SIZE; i++) {
		state[i] = '.';
	}
	state[STATE_SIZE - 1] = 0;
}

int main() {
	char state[STATE_SIZE];

	fgets(state, sizeof("initial state: "), stdin);

	fgets(state + GROWTH_LEEWAY, INITIAL_STATE_SIZE + 1, stdin);
	init_state(state);

	getc(stdin); // newline
	getc(stdin); // newline

	char rules[NUM_RULES][RULE_SIZE]; // pattern (5 chars) + outcome (1 char)
	for (int i = 0; i < NUM_RULES; i++) {
		char line[RULE_LINE_LEN];
		fgets(line, RULE_LINE_LEN, stdin);
		memcpy(rules[i], line, RULE_PATTERN_SIZE);
		rules[i][RULE_OUTCOME] = line[RULE_LINE_LEN - 3];
	}

	char alt_state[STATE_SIZE]; // Will alternate with `state` to be the prev or next.
	init_state(alt_state);

	char *prev_state = state;
	char *new_state = alt_state;
	int num_pots = 0;

	for (int gen = 0; gen < NUM_GENERATIONS; gen++) {
		num_pots = 0;

		for (int i = 0; i < MAX_STATE_SIZE - RULE_PATTERN_SIZE + 1; i++) {
			for (int ri = 0; ri < NUM_RULES; ri++) {
				char *rule = rules[ri];

				if (memcmp(prev_state + i, rule, RULE_PATTERN_SIZE) == 0) {
					new_state[i + 2] = rule[RULE_OUTCOME];
					if (rule[RULE_OUTCOME] == '#') {
						num_pots += i + 2 - GROWTH_LEEWAY;
					}
					break;
				}
			}
		}

		char *tmp = prev_state;
		prev_state = new_state;
		new_state = tmp;
	}

	printf("%d\n", num_pots);
}