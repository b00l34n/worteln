use std::collections::HashSet;
mod file_io;

const PATH_TO_DIC: &str = "./misc/de_DE.dic";

enum Dictionary {
    ListOfWords,
    AmountOfWords,
}

impl Dictionary {
    
    /**
     * Goes through the dictionary file, find the words that have in the disierd length
     * in uppercase and put it into an hash set. It also counts the the amount of found
     * words. Otherwise the set would need to be traversed later again to determen the
     * amount.
     *
     * @word_length: the desierd lenth of the words.
     *
     * @retrun: a tupple consitend of a set with the found words and the number of 
     *          found words (whish should be equal to the length of the set).
     */
    fn new(word_length: usize) -> Dictionary {
    
        use crate::dic::Dictionary::*;

        // the vlaues for the return tupple
        AmountOfWords : usize = 0;
        ListOfWords = HashSet::new();

        // open the dictionary and go through every word:
        for word in file_io::read_file(file_io::open_file(PATH_TO_DIC))
                .split_whitespace() {
            
            // The need to be uppercased befor because of the 
            // way to_uppercase() handles special characters
            let word = word.to_uppercase(); 
            
            // look for words that have the wanted length and shove it into the vector
            if word.char_indices().count() == word_length {
                ListOfWords.insert(word);
                AmountOfWords += 1;
            } 
        }
        Dictionary::{ListOfWords, AmountOfWords} 
    }

    /**
     * returns an arbitrary String from the set of Strings that is would
     * be on the n'th position of an itterartor call.
     *
     * @dic: the String Set
     * @n: the n'th possiotion
     *
     * @return: one of the Words in the set
     */
    fn get_nth_word(&self, n: usize) -> String {
    
        let mut dummy_rator = (ListOfWords).iter();
        let result: String;
    
        match dummy_rator.nth(n) {
            Some(res) => result = res.to_string(),
            None => result = String::from("\0"),
        }
        result
    }
}
