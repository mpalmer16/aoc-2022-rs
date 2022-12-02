# part 1

The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end up with the following list:

```
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
```
This list represents the Calories of the food carried by five Elves:

The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
The second Elf is carrying one food item with 4000 Calories.
The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
The fifth Elf is carrying one food item with 10000 Calories.
In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).

Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

---

Define a transform to read the input text into a vector of strings, splitting on a double newline
```rust
let transform: Vec<String> = |s: String| {
        s.trim()
            .split("\n\n")
            .map(|s| s.to_string())
            .collect()
    };
```
Define a function that can take a vector of strings, then split each string on the newline separator, parse the strings to ints, and sum the values for each,
this will return a new vector of ints
```rust
fn get_elf_counts(elf_strings: Vec<String>) -> Vec<i32> {
    elf_strings
        .iter()
        .map(|s| {
            s.split('\n')
                .map(|s| s.parse::<i32>().expect("could not parse int"))
                .sum::<i32>()
        })
        .collect()
}
```
fetch the input and get the counts, then sort the vector in place - the last value is the largest number
```rust
let input = fetch_with_transform(1, transform);
let mut counts: Vec<i32> = get_elf_counts(input);
counts.sort();
let answer: i32 = counts.last().unwrap()
```