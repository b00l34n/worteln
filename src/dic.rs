use std::collections::HashSet;
mod file_io;

const PATH_TO_DIC: &str = "./misc/de_DE.dic";

/**
 * Goes through the dictionary file, find the words that have in the disierd length
 * in uppercase and put it into an hash set. It also counts the the amount of found
 * words. Otherwise the set would need to be traversed later again to determen the
 * amount.
 *
 * @number_of_letters: the desierd lenth of the words.
 *
 * @retrun: a tupple consitend of a set with the found words and the number of 
 *          found words (whish should be equal to the length of the set).
 */
pub fn get_list_of_words (number_of_letters : usize) -> (HashSet<String>, usize) {

    // the vlaues for the return tupple
    let mut word_count : usize = 0;
    let mut set_of_words = HashSet::new();

    // open the dictionary and go through every word:
    for word in file_io::read_file(file_io::open_file(PATH_TO_DIC))
            .split_whitespace() {
        
        // The need to be uppercased befor because of the 
        // way to_uppercase() handles special characters
        let word = word.to_uppercase(); 
        
        // look for words that have the wanted length and shove it into the vector
        if word.char_indices().count() == number_of_letters {
            set_of_words.insert(word);
            word_count += 1;
        } 
    }
    return (set_of_words, word_count);
}

