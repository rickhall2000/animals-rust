use std::io;
use std::collections::HashMap;

#[derive(Debug)]
enum Question {
    Leaf{name: String, id: i32},
    Branch{question: String, yes: i32, no: i32}
}

enum Result {
    Yes(i32),
    No(i32),
    Win,
    Loss,
}

fn get_answer() -> bool {
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    
    guess.trim() == "yes"
}

impl Question {
    fn ask(&self) {
        match self {
            Question::Leaf{name, id} => {
                println!("Is it a {name}");
                if get_answer() {
                    println!("I win");
                } else {
                    println!("You win");
                }
            },

            Question::Branch{question, yes, no} => {
                println!("{question}");
                if get_answer() {
                    println!("I should continue from here");
                }
                else {
                    println!("I should follow the no branch");
                }
            },
        }
    }
}

fn initial_questions() -> HashMap<i32,Question>{
    let mut questions = HashMap::new();

    let elephant = Question::Leaf {name: String::from("Elephant"), id: 2};
    let first = Question::Branch {question: String::from("Is it a mamal?"), yes: 2, no: 3};
    let shark = Question::Leaf {name: String::from("Shark"), id: 3};
  

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

fn main() {

    let mut questions = initial_questions();

    let x = questions.get(&1).expect("This better work");
    x.ask();

    let x = questions.get(&2).expect("This better work");
    x.ask();

    print_questions(questions);
    println!("All done");

}
