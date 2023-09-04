use std::io;
use std::collections::HashMap;

#[derive(Debug)]
enum Question {
    Leaf{name: String, _id: i32},
    Branch{question: String, yes: i32, no: i32}
}

enum Result {
    Continue(i32),
    Win,
    Lose,
}

fn get_answer() -> bool {
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    
    guess.trim() == "yes"
}

impl Question {
    fn ask(&self) -> Result {
        match self {
            Question::Leaf{name, _id: _} => {
                println!("Is it a {name}");
                if get_answer() {
                    println!("I win");
                    Result::Win
                } else {
                    println!("You win");
                    Result::Lose
                }
            },

            Question::Branch{question, yes, no} => {
                println!("{question}");
                if get_answer() {
                    println!("I should continue from here");
                    Result::Continue(*yes)
                }
                else {
                    println!("I should follow the no branch");
                    Result::Continue(*no)
                }
            },
        }
    }
}

fn initial_questions() -> HashMap<i32,Question>{
    let mut questions = HashMap::new();

    let elephant = Question::Leaf {name: String::from("Elephant"), _id: 2};
    let first = Question::Branch {question: String::from("Is it a mamal?"), yes: 2, no: 3};
    let shark = Question::Leaf {name: String::from("Shark"), _id: 3};
  

    questions.insert(1, first);
    questions.insert(2, elephant);
    questions.insert(3, shark);
    return questions;
}

fn print_questions(questions: HashMap<i32,Question>) 
{
    for question in questions.iter() {
        println!("{:?}", question);
    }
}

fn handle_loss(current_question: &Question) {
    if let Question::Leaf{name, _id} = current_question {
        println!("What was your animal?");

        let mut right_answer = String::new();

        io::stdin() 
        .read_line(&mut right_answer)
        .expect("I want input");

        println!("What is a question that tells the difference between a {right_answer} and a {name}");
        }
}

fn play_game(questions: &HashMap<i32,Question>) {
    let mut current_question = questions.get(&1).expect("This better work");
    
    loop {
        let my_result = current_question.ask();

        match my_result {
            Result::Continue(next) => {
                println!("I should continue with {next}");
                current_question = questions.get(&next).expect("Lookup failed");
            }, 
            Result::Win => { 
                println!("I am done"); 
                break;
            },
            Result::Lose => {
                handle_loss(current_question);
                break;
            }
        };
    }
}

fn main() {

    let questions = initial_questions();
    
    loop {
        play_game(&questions);
        let mut play_again = String::new();
        println!("Would you like to play again?");
        io::stdin()
        .read_line(&mut play_again)
        .expect("I expect readline to work");

        if play_again.trim() != "yes" { break }
    }

    print_questions(questions);
    println!("All done");

}
