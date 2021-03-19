# Course management
I'm studying for a bachelor's degree in computer science.

# Implementation
The project will be in English. I will be using the Rust programming language,
as it is performant and I like using it. It has inbuilt unit test
functionality and should support code coverage reporting.

# Problem & algorithms
The problem is how to reduce the space of data in transit or on disk. I am
going to implement the Huffman coding and Lempel-Ziv compression algorithms.
There are multiple adaptations of Lempel-Ziv, and I'm tentatively going to
implement the original LZ77 variant of it. I'm picking them because different
applications of them are used in numerous ubiquitous file formats such as GIF,
PNG, ZIP, MP3 and JPEG, making it useful common knowledge[[1]](https://en.wikipedia.org/wiki/Huffman_coding#Applications)[[2]](https://en.wikipedia.org/wiki/Lempel-Ziv).

# Usage & input
The program will be a command line application based on the Rust
[[clap]](https://docs.rs/clap/2.33.3/clap/index.html) library. The program will
accept at least one input file, but ideally it would take multiple files and
directories and compress them all. The created archive should be uncompressable
with a different command line argument.

# Data structures
- Arrays will likely be used throughout the program.
- I am mostly likely going to need some sort of list type, since lists are
useful to have under almost any circumstances. An arraylist is both useful and
easy to implement.
- A Huffman tree, which is a sort of binary tree[[3]](https://go-compression.github.io/algorithms/huffman/#tree-building).
LZ also seems to use a tree to remember strings it has encountered before.
- At least some implementations of Huffman coding use a priority queue[[4]](https://en.wikipedia.org/wiki/Huffman_coding#Compression).

# Time efficiency
Huffman coding should be possible to implement in O(n) time[[5]](https://stackoverflow.com/questions/6189765/big-o-complexities-of-algorithms-lzw-and-huffman).
Lempel-Ziv should also be doable in O(n) time, since it only goes through the
input once[[6]](https://go-compression.github.io/algorithms/lz/). Search results
elude me, and some Lempel-Ziv implementations could somewhy take O(n log n).

# Space efficiency
The data structures used to store the different strings grow proportionately to
their input size, depending on how little the data repeats.
