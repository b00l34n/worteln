mod file_io

/**
 *
 */
pub fn get_list_of_words (number_of_letters : usize) -> (Vec<String>, usize) {

    let mut list_of_words : Vec<String> = vec!();
    let mut word_count : usize = 0;
    // go through every word in the dictionary
    for word in file_io::read_dic_file.split_whitespaces() {
        // look for words that have the wanted length and shove it into the vector
        if word.char_indices().count() == number_of_letters {
            list_of_words.push(word);
            word_count += 1;
        } 
    }
    return (list_of_words, word_count);
}

