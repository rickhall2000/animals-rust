use std::io;
use std::collections::HashMap;

#[derive(Debug)]
enum Question {
    Leaf{name: String, id: i32},
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
            Question::Leaf{name, id: _} => {
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

struct QuestionList {
    questions: HashMap<i32, Question>,
    next_question_id: i32,
}

impl QuestionList {
    fn new() -> QuestionList {
        let mut questions = HashMap::new();

        let elephant = Question::Leaf {name: String::from("Elephant"), id: 2};
        let first = Question::Branch {question: String::from("Is it a mamal?"), yes: 2, no: 3};
        let shark = Question::Leaf {name: String::from("Shark"), id: 3};
    
        questions.insert(1, first);
        questions.insert(2, elephant);
        questions.insert(3, shark);

        QuestionList {questions, next_question_id: 4}
    }

    fn add_question(&mut self, question: Question) -> i32 {
        let id = self.next_question_id;
        self.next_question_id += 1;
        self.questions.insert(id, question);
        id
    }

    fn get_next_question_id(&self) -> i32 {
        self.next_question_id
    }

    fn get_question(&self, id: i32) -> Option<&Question> {
        self.questions.get(&id)
    }

    fn get_question_mut(&mut self, id: i32) -> Option<&mut Question> {
        self.questions.get_mut(&id)
    }

    fn print_questions(&self) {
        for question in self.questions.iter() {
            println!("{:?}", question);
        }
    }
}


fn handle_loss(current_question: &Question, next_id: i32) -> (Question, Question) {
    if let Question::Leaf{name, id} = current_question {

            println!("What was your animal?");
            let mut right_answer = String::new();
            io::stdin() 
            .read_line(&mut right_answer)
            .expect("I want input");

            println!("What is a question that tells the difference between a {right_answer} and a {name}");
            let mut new_question_text = String::new();
            io::stdin()
            .read_line(&mut new_question_text)
            .expect("I want input");
            
            println!("What is the right answer for a {right_answer}?");
            let mut new_answer = String::new();
            let answer_yes = new_answer.trim() == "yes";

            io::stdin()
            .read_line(&mut new_answer)
            .expect("I want input");
 
            let yes_id = if answer_yes { next_id } else { *id };
            let no_id = if answer_yes { *id } else { next_id };

            let new_leaf = Question::Leaf{name: String::from(&right_answer), id: next_id};
            let new_branch = Question::Branch{question: String::from(&new_question_text), 
                                                            yes:  yes_id, 
                                                            no: no_id};
        (new_leaf, new_branch)
    }   
    else {
        panic!("I should have a leaf here");
    }
}

fn play_game(questions: &mut QuestionList) {
    let mut current_question = questions.get_question(1).expect("This better work");
    
    loop {
        let my_result = current_question.ask();

        match my_result {
            Result::Continue(next) => {
                println!("I should continue with {next}");
                current_question = questions.get_question(next).expect("Lookup failed");
            }, 
            Result::Win => { 
                println!("I am done"); 
                break;
            },
            Result::Lose => {
                let next_id = questions.get_next_question_id();
                let (new_leaf, new_branch) = handle_loss(current_question, next_id);
                let new_leaf_id = questions.add_question(new_leaf);
                let new_branch_id = questions.add_question(new_branch);
                // this doesn't work, I need to update the last branch to point to the new branch
                break;
            }
        };
    }
}

fn main() {

    let mut questions = QuestionList::new();

    loop {
        play_game(&mut questions);
        let mut play_again = String::new();
        println!("Would you like to play again?");
        io::stdin()
        .read_line(&mut play_again)
        .expect("I expect readline to work");

        if play_again.trim() != "yes" { break }
    }

    questions.print_questions();
    println!("All done");

}
