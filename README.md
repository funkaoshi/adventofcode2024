# Advent of Code 2024

I have a job now, so I can't imagine i'll make it too far with [Advent of Code][1]
this time around, but I do want to learn Rust, so maybe this will all balance
out.

Currently you can `cargo run` inside the folder for each day to get the answers
for that days problems. `cargo test` will run the the code against the test
input.

## Commentary

[### Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)

A nice straightforward problem to get things going. For part 2, I learned that
`into_iter` consumes what it iterates over, so you can't iterate over the same
vector twice. My original solution cloned the vector before iterating. The
smarter solution was to make a histogram you can consult when calculating the
required score. Also learned `include_str!` is a thing. Rust isn't quite as
cumbersome as I had feared for doing these sorts of puzzles.

[### Day 2: Red-Nosed Reports](https://adventofcode.com/2024/day/2)

Another fairly straight forward problem, though it feels like there is probably
a nicer solution than what I built. Learned a little bit about slices in the
process of trying to figure out how to generate combinations of each report for
part 2.

[### Day 3: Mull It Over](https://adventofcode.com/2024/day/3)

Esentially just a Regex question. Useful since I needed to learn how Regex works
in Rust. Am curious for part 2 if there is a tidier way to generate the filtered
list of `mul(x,y)` operations.

[### Day 4: Ceres Search](https://adventofcode.com/2024/day/4)

...

[### Day 5: Print Queue](https://adventofcode.com/2024/day/5)

...

[### Day 6: Guard Gallivant](https://adventofcode.com/2024/day/6)

Initial solution is probably the most naive one, just walking the map
recursively. It doesn't feel like it'll work out for the second part of this
problem, which I have yet to do!

[### Day 7: Bridge Repair](https://adventofcode.com/2024/day/7)

Pretty straightforward recursive solution. I'm curious if this would be too
slow with a language like Python. It's 1:00AM, so should think about whether
you can start the process off in a way that feels less goofy.

[### Day 8: Resonant Collinearity](https://adventofcode.com/2024/day/8)


Again, what feels like a naive solution. Will revist later. Just trying not to
fall behind!


[1]: https://adventofcode.com/2024
