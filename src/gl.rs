use std::io;

pub const AMOUNT_OF_LETTERS: usize = 5;
pub const AMOUNT_OF_TRYS: usize = 5;

/**
 * this method takes the users answer and compares it with the Word that
 * is to be guessed. It returns a string that indicates if a user given
 * letter is in the searched word and if it is at the right possition.
 * 
 *     Letter not in the word:    '-'
 *     Letter in the word:        'o'
 *     Letter in the right place: The letter itself
 * 
 * @users_answer: a borrowed String that contains what will be compared with the
 *                searched word.
 * @correct_answer: a borrowed String containing the searched word.
 * @return: A tupple with a String that shows the the compare result and a boolean
 *          that indicates, if the Strings are equal.
 */
pub fn string_analysis ( users_answer: &str, correct_answer: &str) -> (String, bool) {

    let mut result_string : String = String::new(); 
    let mut _index : usize = 0;

    // The extra check at the begging is a fix for the Double Letter Bug
    for (_index, current_user_char) in users_answer.chars().enumerate() {
        // First check if the chars at _index are the same 
        if correct_answer[_index..(_index+1)].contains(current_user_char) {
                    result_string.push(current_user_char);
        } else {
            // Then check if the char can be found anywhere else in the searchd word.
            match correct_answer.find(current_user_char) {
                Some(_n) => {
                    result_string.push('o');
                }
                None => {
                    result_string.push('-');
                } 
            }
        }
    }
    let eql = correct_answer.eq(&result_string); 
    (result_string, eql)
}
/**
 * Reads the User Answer form StdIn, removes the linefeed and cuts it to size.
 * After that, it returns a uppercase coppy of the Input.
 *
 * @return: The formated user input
 */
pub fn user_input() -> String {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer){
        Ok(_n) => {
            buffer.remove(buffer.find('\n').unwrap());
            buffer.truncate(AMOUNT_OF_LETTERS);
            buffer.to_ascii_uppercase() 
        }
        Err(_error) => {
            println!("Error while reading user Input"); 
            String::from("\0")
        } 
    }
}

/**
 * Prints a sentense, that shows the user how many trys they have left.
 *
 * @t: the amount of trys left
 */
pub fn print_round_announcment(t : usize){
    if t == 0 {
        println!("Sie haben {} Versuche",AMOUNT_OF_TRYS);
    } else if (AMOUNT_OF_TRYS - t) > 1 {
            println!("Sie haben noch {} Versuche",AMOUNT_OF_TRYS - t);
    } else {
            println!("Sie haben noch einen Versuche");
    }
}

/** 
 * this method prints the users word followed by the copmare results
 * right underneath it.
 * 
 * @a: a borrowed vector containing the user input strind,
 *     the compare result string and a boolean indicating if they are
 *     equal.
 * @t: the amount tryes.
 */
pub fn print_answer_block( a: &[(String, String, bool)], t: usize) {
        if t > 0 {
        println!("\nDeine Antworten waren:\n"); 
        for i in 0..t {
            let tup = a.get(i).unwrap();
            println!("{}", tup.0);
            print!("{}", tup.1);
            if tup.2 {
                println!(" -> DONE");
            } else {
                println!();
            }
        }
    }
    println!();
}
