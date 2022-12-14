use {
    std::collections::HashMap,
    rand::Rng,
};

use crate::{
    helpers, 
    answers
};

#[derive(Clone)]
struct Quiz {
    id: u64, // Starts from 0
    name: String, // example: "Rust lang", "Mathematics"
    description: String, // This is a description of a question ......
    answer: answers::Choices, // "a" | "b" | "c" | "d" (or)
    choices: Vec<String>, // Vectors of multiple choices
    asked: bool // to prevent duplicates
}

#[derive(Clone)]
pub struct Quizzes {
    list: HashMap<u64, Quiz>, 
    pub correct: u64, // Number of questions answered correctly
    pub questions_asked: u64 // Number of questions that has been asked
}

impl Quizzes {
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
            correct: 0,
            questions_asked: 0
        }
    }

    /// Add quiz to list
    pub fn add(&mut self, id: u64, name: &str, description: &str, answer: answers::Choices, choices: Vec<String>) {
        let quiz = Quiz {
            id,
            name: name.to_string(),
            description: description.to_string(),
            answer,
            choices,
            asked: false
        };
        self.list.insert(id, quiz);
    }

    // TODO: Deref Unwrap to Clone... ?
    /// Returns a random, yet to be asked question
    fn get_unasked_question(&self) -> Option<Quiz> {
        let quiz: Vec<_> = self.list.iter().filter(|v| !v.1.asked).map(|v| v.1).collect();
        if !quiz.is_empty() {
            let rng = rand::thread_rng().gen_range(0..quiz.len());
            return Some((*quiz.get(rng).unwrap()).clone());
        }
        None
    }

    /// Prints question, first the question's name, then the description (detail), then provide the multiple choices
    fn print_pertanyaan(&self, quiz: &Quiz, quiz_number: usize) {
        println!("{}", quiz.name);
        println!("{}. {}", quiz_number, quiz.description);
        let mut curr_option: u8 = 65;
        for i in &quiz.choices {
            println!("{}. {}", curr_option as char, i);
            curr_option += 1;
        }
    }

    /// Uses get_unasked_question to get random questions which it then outputs
    ///
    /// If an unasked question is found, it will mark it as an asked question
    /// 
    /// Uses print_pertanyaan to output the question
    pub fn ask(&mut self, num_of_questions: usize) {
        for i in 0..num_of_questions {
            let mut question = match self.get_unasked_question() {
                Some(quiz) => quiz,
                None => {
                    println!("Tidak ada pertanyaan lagi yang dapat ditanyakan");
                    return;
                },
            };
            self.print_pertanyaan(&question, i + 1);
            let answered = helpers::get_char_input("Input ['A' | 'B' | 'C' | 'D']: ", 'A', 'D', true);
            if answers::to_choices_enum(&answered.to_string()).unwrap() == question.answer{
                self.correct += 1;
            }
            question.asked = true;
            self.list.insert(question.id, question);
            self.questions_asked += 1
        }
    }
}