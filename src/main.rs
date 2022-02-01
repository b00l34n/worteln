use rand::Rng;

mod game_logic;
mod dic;

fn main() {
    // Get the modifierd dictionary read
    let dictionary = dic::get_list_of_words(game_logic::AMOUNT_OF_LETTERS);
    
    // Get a random number for the word to guess 
    let random_number = rand::thread_rng().gen_range(0..(dictionary.1) - 1); 
   
    // A random word from the dictionary
    let word_to_guess: String = dic::get_nth_word(&dictionary.0,random_number); 
 
    // init the accumulator vector for all user answer
    let mut user_answers: Vec<(String, String, bool)>
        = vec![(String::new(), String::new(), false); game_logic::AMOUNT_OF_TRYS];
        
    println!("Erraten sie das Wort!\nEs besteht aus 5 Buchstaben.");
    
    for rounds in 0 .. game_logic::AMOUNT_OF_TRYS {
        game_logic::print_round_announcment(rounds);  
        game_logic::print_answer_block(&user_answers, rounds);

        // get user input
        let answer = game_logic::user_input();
        let analyzed_answer = game_logic::string_analysis(&answer,&word_to_guess);
        
        // store user intput string in vector
        let this_rounds_answer = (answer,analyzed_answer.0,analyzed_answer.1);
        user_answers.insert(rounds, this_rounds_answer);
        
    } 
    game_logic::print_answer_block(&user_answers, game_logic::AMOUNT_OF_TRYS);
    println!("{}",word_to_guess);
}

