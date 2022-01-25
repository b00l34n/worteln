use std::ops::Add;

const AMOUNT_OF_TRYS: usize = 5;
const AMOUNT_OF_LETTERS: usize = 5;

fn round_Announcer(t : usize){
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
    println!("\nDeine Antworten waren:\n"); 
    if t > 0 {
        for i in 0..t {
            println!("{}", a.get(i).unwrap());
        }
    }
    println!();
}

fn main() {

    let mut userAnswers: Vec<String> = vec![String::new(); AMOUNT_OF_TRYS];

    println!("Erraten sie das Wort!\nEs besteht aus 5 Buchstaben.");
    for rounds in 0 .. AMOUNT_OF_TRYS {
        round_Announcer(rounds);  

        println!("\n <DB> User input! </DB>");

        let answer = userAnswers.get_mut(rounds);
        let testStr = "Test: ".to_string() + rounds.to_string().trim();
        answer.unwrap().push_str(testStr.trim()); 
        print_user_answers(&userAnswers, rounds);
    }

}
