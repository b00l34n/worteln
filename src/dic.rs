use std::collections::HashSet;
mod file_io;

pub struct Dictionary {
    // make them privat again through get/set methods?
    pub word_list : HashSet<String>,
    pub word_count : usize,
}

/**
 * Goes through the dictionary file, find the words that have in the disierd length
 * in uppercase and put it into an hash set. It also counts the the amount of found
 * words. Otherwise the set would need to be traversed later again to determen the
 * amount.
 *
 * @dic_path: a string containing the path to a dic file 
 * @word_length: the desierd lenth of the words.
 *
 * @retrun: a dictionary struct containing the amount of words and the Set of found words 
 */
pub fn new(dic_path: &str, word_length: usize) -> Dictionary {
    
    // init the vlaues for the return tupple
    let mut word_count: usize = 0;
    let mut word_list: HashSet<String> = HashSet::new(); 

    // open the dictionary and go through every word:
    for word in file_io::read_file(file_io::open_file(dic_path))
            .split_whitespace() {
  
        // The need to be uppercased befor because of the 
        // way to_uppercase() handles special characters
        let word = word.to_uppercase(); 
    
        // look for words that have the wanted length and shove it into the vector
        if word.char_indices().count() == word_length {
            word_list.insert(word);
            word_count += 1;
        } 
    }
    Dictionary {word_list, word_count} 
}

/**
 * Checks if the word is in the Dictionary eg. if it is a real word.
 *
 * @word: the word to look for
 * @d: the dictionary to search the word in
 *
 * @return: ture, if the word is found in the dictionary
 */
pub fn is_a_word(word: &String, d: &Dictionary) -> bool{
   return d.word_list.contains(word); 
}

/**
 * returns an arbitrary String from the set of Strings that is would
 * be on the n'th position of an itterartor call.
 *
 * @d: the dictionary containing a set of words 
 * @n: the n'th possiotion
 *
 * @return: one of the Words in the set
 */
pub fn get_nth_word(d: &Dictionary, n: usize) -> String {
    
    let mut dummy_rator = (d.word_list).iter();
    let result: String;

    match dummy_rator.nth(n) {
        Some(res) => result = res.to_string(),
        None => result = String::from("\0"),
    }
    result
}
