
use rand::Rng;
use std::io;

//Struct to hold a question/answer
#[derive(Clone)]
struct Answer{
    category: String,
    text: String,
}

//Struct to hold data about a specific round - question, guessed letters, and score
struct Round{
    answer: Answer,
    guessed: String,
    score: i32,
}

impl Round{    
    fn mask_answer(&self)->String {
        let mut output = String::new();
        for (pos, chr) in self.answer.text.chars().enumerate() {
            output.push(if chr == ' ' {chr} 
                else if self.guessed.contains(chr) {chr}
                else {'_'}
            );
        }
        output
    }

    fn won(&self) -> bool {
        if !(self.mask_answer().contains('_')){
            return true;
        }
        false
    }

    fn update(&mut self) {
        let mut guess = askinput("Guess a letter".to_string());
        self.guessed.push(guess.trim().chars().nth(0).expect("No chars in guess"));
    }

    fn draw(&self) {
        println!("{}", self.mask_answer());
    }
}

//Outputs questiontext and then returns a readed string from stdin
fn askinput(questiontext: String)->String{
    let mut input = String::new();
    println!("{}", questiontext);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.truncate(input.len()-1);
    input
}

//Draws an answer with the text masked
fn draw_answer(answer: String, guessed: String){

}

fn draw(){

}

fn main() {
    let mut rng = rand::thread_rng();
    let mut name = String::new();

    println!("Wheel of Fortune");

    let answers = [
        Answer{category: "Dogs".to_string(), text: "german shepard".to_string()},
        Answer{category: "Airplanes".to_string(), text: "c17 globemaster".to_string()}
    ];

    name = askinput("What is your name".to_string());
    println!("Your name is {}", name);
    
    let mut answernumber = rng.gen_range(0..answers.len());
    println!("{}", answernumber);

    println!("The category is: {}", answers[answernumber].category.to_string());
    let mut round = Round { answer: answers[answernumber].clone(), guessed: "a".to_string(), score: 0 };
    round.draw();

    while !(round.won()){
        round.update();
        round.draw();
    }
    
    println!("The answer was: {}", round.answer.text);
    println!("Congratulations!!");
}
