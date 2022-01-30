mod game_logic;
mod dic;

fn main() {

    // Get the modifierd dictionary read
    let dictionary = dic::get_list_of_words(game_logic::AMOUNT_OF_LETTERS);
    println!("Amount of words: {}", dictionary.1);
    println!("some words:");

    for c in 1..(dictionary.1) {
        if (&dictionary.1 % c) == 5 {
            println!("{}", dic::get_nth_word(&dictionary.0, c));
        } 
    } 
/*
    let mut user_answers: Vec<(String, String, bool)>
        = vec![(String::new(), String::new(), false); game_logic::AMOUNT_OF_TRYS];
  
    // Placeholder for when the Random Word Picker is done
    let word_to_guess: String = String::from("DUMMY");
        
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
*/
}
