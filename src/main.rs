use std::io;

const AMOUNT_OF_TRYS: usize = 5;
const AMOUNT_OF_LETTERS: usize = 5;

fn round_announcer(t : usize){
    if t == 0 {
        println!("Sie haben {} Versuche",AMOUNT_OF_TRYS);
    } else {
        if (AMOUNT_OF_TRYS - t) > 1 {
            println!("Sie haben noch {} Versuche",AMOUNT_OF_TRYS - t);
        } else {
            println!("Sie haben noch einen Versuche");
        }
    }
}


/**
 * Reads the User Answer form StdIn, removes the linefeed and cuts it to size.
 * After that, it returns a uppercase coppy of the Input.
 *
 * @return: The formated user input
 */
fn user_input() -> String {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer){
        Ok(_n) => {
            buffer.remove(buffer.find("\n").unwrap());
            buffer.truncate(AMOUNT_OF_LETTERS);
            return buffer.to_ascii_uppercase(); 
        }
        Err(_error) => {
            println!("Error while reading user Input"); 
            return String::from("\0");
        } 
    }
}
fn print_answer_block( a: &Vec<(String, String, bool)>, t: usize) {
       if t > 0 {
        println!("\nDeine Antworten waren:\n"); 
        for i in 0..t {
            let tup = a.get(i).unwrap();
            println!("{}", tup.0);
            print!("{}", tup.1);
            if tup.2 {
                println!(" -> {}", tup.2);
            } else {
                println!();
            }
        }
    }
    println!();
}

fn string_analysis ( users_answer: &String, correct_answer: &String) -> (String, bool) {

    // Placeholder
    return (String::from("-----"), correct_answer.eq(users_answer));

}

fn main() {

    let mut user_answers: Vec<(String, String, bool)>
        = vec![(String::new(), String::new(), false); AMOUNT_OF_TRYS];
  
    // Placeholder for when the Random Word Picker is done
    let word_to_guess: String = String::from("DUMMY");
        
    println!("Erraten sie das Wort!\nEs besteht aus 5 Buchstaben.");
    
    for rounds in 0 .. AMOUNT_OF_TRYS {
        round_announcer(rounds);  
        print_answer_block(&user_answers, rounds);

        // get user input
        let answer = user_input();
        let analyzed_answer = string_analysis(&answer,&word_to_guess);
        
        // store user intput string in vector
        let this_rounds_answer = (answer,analyzed_answer.0,analyzed_answer.1);
        user_answers.insert(rounds, this_rounds_answer);
        
    }
    
    print_answer_block(&user_answers, AMOUNT_OF_TRYS);

}
