use std::io;

const AMOUNT_OF_TRYS: usize = 5;
const _AMOUNT_OF_LETTERS: usize = 5;

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

fn print_user_answers( a: &Vec<String>, t: usize) {
    if t > 0 {
        println!("\nDeine Antworten waren:\n"); 
        for i in 0..t {
            println!("{}", a.get(i).unwrap());
        }
    }
    println!();
}

fn user_input() -> String {

    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer){
        Ok(_n) => {
            return buffer;
        }
        Err(_error) => {
            println!("Error while reading user Input"); 
            return String::from("\0");
        } 
    }
}

fn main() {

    let mut user_answers: Vec<String> = vec![String::new(); AMOUNT_OF_TRYS];

    println!("Erraten sie das Wort!\nEs besteht aus 5 Buchstaben.");
    for rounds in 0 .. AMOUNT_OF_TRYS {
        round_announcer(rounds);  
        print_user_answers(&user_answers, rounds);
       
        user_answers.get_mut(rounds).unwrap().push_str(user_input().trim());

    }

}
