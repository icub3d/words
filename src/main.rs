use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use icub3d_combinatorics::Combination;
use icub3d_combinatorics::Permutation;

fn main() -> io::Result<()> {
    // Read our word list.
    let mut words = HashSet::new();
    let file = File::open("/usr/share/dict/american-english")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        words.insert(line?);
    }

    // Get our letters.
    let letters: Vec<String> = env::args().collect();
    let letters: Vec<char> = letters.get(1).unwrap().chars().collect();

    // Loop through all permutations.
    let mut seen = HashSet::new();
    for x in 3..letters.len() + 1 {
        for c in Combination::new(letters.len(), x) {
            for p in Permutation::new(c.len()) {
                let mut s = String::new();
                for i in p.iter() {
                    s.push(letters[c[*i]]);
                }

                if words.contains(&s) && !seen.contains(&s) {
                    seen.insert(s.clone());
                    println!("{}", s);
                }
            }
        }
    }
    Ok(())
}
