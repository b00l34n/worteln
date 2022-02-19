use std::io;
use std::collections::HashMap;

pub const AMOUNT_OF_LETTERS: usize = 5;
pub const AMOUNT_OF_TRYS: usize = 5;

/*
struct Statistic {
    values : HashMap<char, usize>,
}
impl Statistic {
    pub fn new() -> Self {
        let v: HashMap<char, usize> = HashMap::new();
        Self {
            values: v,
        }
    }
} 
*/


struct Answer {
    word: String,
    statistic: HashMap<char, usize>,      
} 
impl Answer {

    pub fn new(word: String) -> Self {
        Self {
           word,
           statistic: gen_stat(&word), 
        }
    }

}

/**
 * this is a helper mehtod to collect statistical data of a given string.
 * It counts the amounts of every character and puts the value in a hash
 * map with the keys being the corosponding chars.
 * This method is meant for the initialization of the Answer Struct.
 *
 * @w: a referce to the string that is supposed to be analysed
 *
 * @return: the hashmap containing the statistical data about the string 
 */
fn gen_stat (w: &String) -> HashMap<char, usize> {
    let mut stat : HashMap<char, usize> = HashMap::new();
    
    // Go through every char in the word
    for c in w.chars() {
        
        match stat.get(&c) {
            Some(v) => { 
                // If the char was alread found at least once,
                // increase the value at the char by one 
                stat.remove(&c);
                stat.insert(c, v + 1);
            }
            None => {
                // If the char is not yet in the statistic
                // create a new entry with the value 1
                stat.insert(c, 1);
            }
        } 
    }
    stat
}


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
 * 
 * @return: A tupple with a String that shows the the compare result and a boolean
 *          that indicates, if the Strings are equal.
 */
pub fn string_analysis ( users_answer: &str, correct_answer: &str) -> (String, bool) {
   
    // TODO: Word Analysis
    //      If doubleletters are given but the letter appears only once in the searched word
    //      then only one of them will be marked as "in the word" instead of both.
    //      Exaple:
    //              User Input: COOLS
    //              Analyisi:   -O--o
    //        
    //      The because the searched word doesn't have a second 'O' in it, the second 'O' in
    //      the user input will be marked as "not in the word" as well.
    let mut result_string : String = String::new(); 
    let mut _index : usize = 0;

    if users_answer.len() == AMOUNT_OF_LETTERS {
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
