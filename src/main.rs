use rand::Rng;

mod gl;
mod dic;

const PATH_TO_DIC: &str = "./misc/de_DE.dic";

fn main() {
    // Get the modifierd dictionary read
<<<<<<< HEAD
    let dictionary = dic::new(PATH_TO_DIC, game_logic::AMOUNT_OF_LETTERS);
=======
    let dictionary = dic::get_list_of_words(gl::AMOUNT_OF_LETTERS);
>>>>>>> src/gl@wordAnalysis
    
    // Get a random number for the word to guess 
    let random_number = rand::thread_rng().gen_range(0..(dictionary.word_count) - 1); 
   
    // A random word from the dictionary
    let word_to_guess: String = dic::get_nth_word(&dictionary,random_number); 
 
    // init the accumulator vector for all user answer
    let mut user_answers: Vec<(String, String, bool)>
        = vec![(String::new(), String::new(), false); gl::AMOUNT_OF_TRYS];
        
    println!("Erraten sie das Wort!\nEs besteht aus 5 Buchstaben.");
    
    for rounds in 0 .. gl::AMOUNT_OF_TRYS {
        gl::print_round_announcment(rounds);  
        gl::print_answer_block(&user_answers, rounds);

<<<<<<< HEAD
        let mut answer : String; 

        // get user input until it's a word
        loop {
            answer = game_logic::user_input();
            if dic::is_a_word(&answer, &dictionary) {
                break;
            } else {
                println!("{} is not a word", answer);
            } 
        } 
   
        let analyzed_answer = game_logic::string_analysis(&answer,&word_to_guess);
=======
        // get user input
        let answer = gl::user_input();
        let analyzed_answer = gl::string_analysis(&answer,&word_to_guess);
        
>>>>>>> src/gl@wordAnalysis
        // store user intput string in vector
        let this_rounds_answer = (answer,analyzed_answer.0,analyzed_answer.1);
        user_answers.insert(rounds, this_rounds_answer);
        
    } 
    gl::print_answer_block(&user_answers, gl::AMOUNT_OF_TRYS);
    println!("{}",word_to_guess);
}

