use std::io;
use std::collections::HashMap;

#[derive(Debug)]
enum Question {
    Leaf{name: String, id: i32},
    Branch{question: String, yes: i32, no: i32}
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
    println!("{:?}", questions);
}

fn main() {
    let mut guess = String::new();

    println!("What is your answer?");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Hello, {guess}");

    let mut questions = initial_questions();
    print_questions(questions);
    println!("All done");

}
