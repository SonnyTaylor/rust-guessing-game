use rand::seq::SliceRandom;
use std::io::{self, Write};
struct Question {
    question: String,
    options: Vec<String>,
    answer: String,
}

fn ask_question(question: &Question) -> io::Result<bool> {
    println!("{}", question.question);
    for option in &question.options {
        println!("{}", option);
    }

    let mut user_answer = String::new();
    print!("Enter your answer (A, B, C, D): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut user_answer)?;

    Ok(user_answer.trim().to_uppercase() == question.answer)
}

fn main() -> io::Result<()> {
    let mut questions = vec![
        Question {
            question: "What is the output of this code in python: print(3 + '3')".to_string(),
            options: vec![
                "A. 6".to_string(),
                "B. 33".to_string(),
                "C. TypeError".to_string(),
                "D. '6'".to_string(),
            ],
            answer: "C".to_string(),
        },
        Question {
            question: "Which programming language is often used for frontend web development?"
                .to_string(),
            options: vec![
                "A. Python".to_string(),
                "B. Java".to_string(),
                "C. Ruby".to_string(),
                "D. JavaScript".to_string(),
            ],
            answer: "D".to_string(),
        },
        Question {
            question: "What does OOP mean?".to_string(),
            options: vec![
                "A. Original Objectified Python".to_string(),
                "B. Object Orginated Programs".to_string(),
                "C. Object-oriented programming".to_string(),
                "D. Orange Orangatang Pie".to_string(),
            ],
            answer: "C".to_string(),
        },
        Question {
            question: "What is the correct symbol for adding a comment in python?".to_string(),
            options: vec![
                "A. //".to_string(),
                "B. #".to_string(),
                "C. <!-- -->".to_string(),
                "D.  /* */".to_string(),
            ],
            answer: "B".to_string(),
        },
        Question {
            question: "Who created python?".to_string(),
            options: vec![
                "A. Guido van Rossum".to_string(),
                "B. Linus Torvalds".to_string(),
                "C. Terry A. Davis".to_string(),
                "D. Tim Berners-Lee".to_string(),
            ],
            answer: "A".to_string(),
        },
        Question {
            question: "In Python, which keyword is used to define a function?".to_string(),
            options: vec![
                "A. define".to_string(),
                "B. func".to_string(),
                "C. def".to_string(),
                "D. function".to_string(),
            ],
            answer: "C".to_string(),
        },
        Question {
            question: "Which of the following is not a valid Python data type?".to_string(),
            options: vec![
                "A. int".to_string(),
                "B. double".to_string(),
                "C. list".to_string(),
                "D. tuple".to_string(),
            ],
            answer: "B".to_string(),
        },
    ];

    let mut score = 0;
    questions.shuffle(&mut rand::thread_rng());

    for (i, question) in questions.iter().enumerate().take(5) {
        println!("Question {} of 5", i + 1);

        if ask_question(question)? {
            println!("Correct!\n");
            score += 1;
        } else {
            println!("Wrong! The correct answer is {}.\n", question.answer);
        }
    }

    println!("You scored {}/5.", score);

    Ok(())
}
