use proconio::input;
use proconio::marker::{Chars};
use std::io::{stdout, Write};

#[derive(Debug)]
struct Word {
    word: Vec<char>
}

impl Word {
    fn print_diff(&self, word: &Word) {
        for (i, char) in self.word.iter().enumerate() {
            let maybe_correct_char = word.word.get(i);
            match maybe_correct_char {
                Some(correct_char) => {
                    if char == correct_char {
                        println!("Correct");
                    } else {
                        println!("Incorrect");
                    }
                },
                None => {}
            }
            // print!("{}", *char);
            // stdout().flush().unwrap();
        }
        // print!("\n");
    }
}

fn main() {
    let correct_answer = Word { word: ['w', 'o', 'r', 'd', 'l', 'e'].to_vec()};

    for _i in 0..5 {
        input! {
            chars: Chars
        }

        let input = Word { word: chars};

        input.print_diff(&correct_answer);
    }
}
