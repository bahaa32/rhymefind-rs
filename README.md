# Rhymefind-rs

This is a port of one of my CS assignments in Rust to improve performance. Here's an xkcd-style chart to show the improvement when starting up, reading the dictionary and finding rhymes for 300 unique words:

<img src="https://user-images.githubusercontent.com/62975113/113674024-8d87fa80-966e-11eb-9024-2fdc10105501.png" alt="Rust vs Python implementation performance" width="600"/>

As with my [go-greenscreen](https://github.com/beldin222/go-greenscreen/) project, the performance improvement wasn't initially as good as the final version. I used `cargo flamegraph`, `criterion-rs` and `perf` to measure areas which could improve and changes in performance, although the numbers in the chart are from `time` but they were fairly consistent in my experience. I plan to eventually push my first and second attempts at this but for now here's another _very_ scientific chart to show the improvements (xkcd-style charts are my favorites).

<img src="https://user-images.githubusercontent.com/62975113/113673953-7812d080-966e-11eb-9495-b0e960311065.png" alt="Performance at each attempt" width="600"/>

## Lessons Learned
- Second attempt: Parsing data into the `Phonemes` struct once when loading the dictionary is faster than parsing for each word for large numbers of words (_duh_). It's a tradeoff between a quick startup and slow search vs slow startup and quick search (slow startup here is about 189ms on my machine so it's worth it IMO)
- Final attempt: Using `filter` and `map` was significantly faster than using for loops for some reason. Also, I managed to reduce run time by about 250ms by avoiding things I didn't _really_ need and looking into how some of the functions work. For instance, I was collecting `word_rhymes` into a vector for each pronunciation of the input word before using `append` to add it to the final result. Using `extend` without collecting `word_rhymes` (leaving it as an iterator) gave me a nice performance boost.

## Usage
First input line should be the path to a dictionary of format similar to `WordToPhonemes.txt` and every line after that can be a word to search. To exit, type `exit`.
