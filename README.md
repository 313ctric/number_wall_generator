# A library for computing number walls

## What are number walls?

A good introductory video by Mathologer on [YouTube](https://youtu.be/NO1_-qptr6c).

A very detailed explanation and proof of the various algorithms for generating number walls is available [here](https://cs.uwaterloo.ca/journals/JIS/VOL4/LUNNON/numbwall10.html).

A paper relating to properties of number walls that includes many examples of them which are used in the tests and examples is available [here](https://doi.org/10.48550/arXiv.0906.3286).

## Using the library

The best explanation is to look at some of the code in the [examples](/examples/) directory.

In short:
1. Initialise a holder with a sequence (or function), and bounds as to which values you would like from that wall.
2. Call the `calculate_next_line` function to calculate the next line of the wall.
3. Get the contents of that line with `get_last_line`.
4. Repeat steps 2 and 3 until all of your requested lines have been calculated, and `calculate_next_line` returns `None`.

There are 3 types of wall available:
 - bi directional: the most traditional form of number wall. A sequence (or function) is used for positive and negative values.
 - left const: these walls have constant values in positions -2 and -1, this allows for positive only sequences (or functions) to be calculated, although you may get unexpected or strange results below the leading diagonal.
 - repeating sequence: optimised for periodic sequences, the left and right edges of the wall wrap around to each other, to allow fewer values to be computed.

## What can it do?

See the examples in the [images](/images/) directory.

## Other notes

Only the last line is available as once a line is no longer needed for future calculations it is dropped to save memory.

The program is fairly fragile, most input errors are handled by `panic!`.

You may get errors using strange left values for the left const wall, as they could potentially create invalid walls (maybe?), however I haven't tested this.

## ToDo

Add a `WallHolder` trait including the `calculate_next_line`, `get_last_line`, `get_line_count` and `get_line_memory` functions to reduce code reuse for programs using multiple types of wall.
