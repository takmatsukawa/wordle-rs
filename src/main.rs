use colored::*;
use proconio::input;
use proconio::marker::Chars;

#[derive(Debug)]
struct Word {
    word: Vec<char>,
}

impl Word {
    fn print_diff(&self, other: &Word) {
        for (i, char) in self.word.iter().enumerate() {
            let maybe_correct_char = other.word.get(i);
            match maybe_correct_char {
                Some(correct_char) => {
                    if char == correct_char {
                        print!("{}", char.to_string().green());
                    } else if other.word.contains(char) {
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
        self.word == other.word
    }
}

fn main() {
    let correct_answer = Word {
        word: ['w', 'o', 'r', 'd', 'l', 'e'].to_vec(),
    };

    let mut correct = false;

    for _i in 0..5 {
        input! {
            chars: Chars
        }

        let input = Word { word: chars };

        input.print_diff(&correct_answer);
        if input.eq(&correct_answer) {
            println!("correct answer!");
            correct = true;
            break;
        }
    }

    if !correct {
        println!("Answer is {}", correct_answer.word.into_iter().collect::<String>());
    }
}
