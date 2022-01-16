use colored::*;
use proconio::input;
use proconio::marker::Chars;
use ansi_escapes::{EraseLines};

#[derive(Debug)]
struct Word(Vec<char>);

impl Word {
    fn print_diff(&self, other: &Word) {
        for (i, char) in self.0.iter().enumerate() {
            let maybe_correct_char = other.0.get(i);
            match maybe_correct_char {
                Some(correct_char) => {
                    if char == correct_char {
                        print!("{}", char.to_string().green());
                    } else if other.0.contains(char) {
                        print!("{}", char.to_string().yellow());
                    } else {
                        print!("{}", char);
                    }
                }
                None => {}
            }
        }
        println!();
    }
    
    fn eq(&self, other: &Word) -> bool {
        self.0 == other.0
    }
}

fn main() {
    let correct_answer = Word(['w', 'o', 'r', 'd', 'l', 'e'].to_vec());

    let mut correct = false;

    for _i in 0..5 {
        input! {
            chars: Chars
        }

        let input = Word(chars);

        print!("{}", EraseLines(2));
        input.print_diff(&correct_answer);
        if input.eq(&correct_answer) {
            println!("correct answer!");
            correct = true;
            break;
        }
    }

    if !correct {
        println!("Answer is {}", correct_answer.0.into_iter().collect::<String>());
    }
}
