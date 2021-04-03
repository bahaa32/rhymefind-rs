use std::{
    collections::{hash_map::Entry, HashMap},
    fs::File,
    io::{self, BufRead, BufReader},
    str::SplitAsciiWhitespace,
};

struct Phonemes {
    stress: String,
    before_stress: String,
    after_stress: String,
}

fn main() {
    let mut filename = String::new();
    println!("Enter dictionary path:");
    io::stdin()
        .read_line(&mut filename)
        .expect("Cannot read terminal!");
    let filename = filename.trim_end();
    let phonemes_dictionary = read_dictionary(filename);
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read terminal!");
        let input = input.trim_end().to_uppercase();
        if input == "EXIT" {
            break;
        };
        if !phonemes_dictionary.contains_key(&input) {
            println!(" -- word not in dictionary -- \n");
            continue;
        }
        print_rhymes(&input.to_uppercase(), &phonemes_dictionary);
    }
}

fn read_dictionary(filename: &str) -> HashMap<String, Vec<Phonemes>> {
    let mut spelling_to_phonemes: HashMap<String, Vec<Phonemes>> = HashMap::new();
    // https://stackoverflow.com/questions/25167615/
    //  how-to-combine-reading-a-file-line-by-line-and-iterating
    //  -over-each-character-in
    let file = File::open(filename).expect("Cannot open dictionary file!");
    let file = BufReader::new(file); // Buffering greatly speeds up execution
    for line in file.lines().filter_map(|result| result.ok()) {
        let mut iter = line.trim_end().split_ascii_whitespace();
        let word = iter.next().expect("Encountered invalid dictionary line.");
        if let Some(phonemes) = Phonemes::from_iter(iter) {
            // https://stackoverflow.com/questions/33243784/append-to-vector-as-value-of-hashmap
            match spelling_to_phonemes.entry(String::from(word)) {
                Entry::Vacant(e) => {
                    e.insert(vec![phonemes]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(phonemes);
                }
            }
        };
    }
    // Drop extra capacity since size won't change during lifetime
    spelling_to_phonemes.shrink_to_fit();
    return spelling_to_phonemes;
}

fn print_rhymes(input_word: &str, phonemes_dictionary: &HashMap<String, Vec<Phonemes>>) {
    // let input_word = &input_word.to_uppercase();
    println!("Rhymes for: {}", input_word.to_uppercase());
    let mut rhymes: Vec<&String> = Vec::new();
    for input_phonemes in &phonemes_dictionary[input_word] {
        let word_rhymes = phonemes_dictionary
            .iter()
            .filter(|(_, phonemes)| {
                phonemes
                    .iter()
                    .any(|phoneme| check_rhyme(input_phonemes, phoneme))
            })
            .map(|(word, _)| word);
        rhymes.extend(word_rhymes);
    }
    if !rhymes.is_empty() {
        rhymes.sort();
        for rhyme in rhymes {
            println!(" {}", rhyme)
        }
    } else {
        println!(" -- no results -- ")
    }
    println!("");
}

fn check_rhyme(ref_phoneme: &Phonemes, check_phoneme: &Phonemes) -> bool {
    ref_phoneme.after_stress.eq(&check_phoneme.after_stress)
        && ref_phoneme.stress.eq(&check_phoneme.stress)
        && ref_phoneme.before_stress.ne(&check_phoneme.before_stress)
}

impl Phonemes {
    fn from_iter(mut iter: SplitAsciiWhitespace) -> Option<Phonemes> {
        let mut last: Option<&str> = None;
        for phoneme in iter.by_ref() {
            if phoneme.ends_with('1') {
                if let Some(last) = last {
                    return Some(Phonemes {
                        stress: String::from(phoneme),
                        before_stress: String::from(last),
                        after_stress: iter.collect::<String>(),
                    });
                };
            };
            last = Some(phoneme);
        }
        return None;
    }
}
