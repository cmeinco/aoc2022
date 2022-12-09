use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

// https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

fn main() {
    evaluate_results(star1("./input01-test.txt".to_string()), "15".to_string());
    evaluate_results(star1("./input01-1.txt".to_string()), "-1".to_string()); // 71471
    evaluate_results(star2("./input01-test.txt".to_string()), "12".to_string());
    evaluate_results(star2("./input01-1.txt".to_string()), "-1".to_string()); // 211189
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn evaluate_results(left: String, right: String) {

    if left != right {
        if right == "-1"{
            println!("[FINAL OUTPUT] {}", left);
        }else{
            println!("[FAIL] DoesNotMatch {} != {} ", left, right);
        }
    }else{
        println!("[PASS] {} ", left);
    }
}

fn star1(file_path: String) -> String {
    println!("\n{} {}", function!(), file_path);
    
    let mut outside_counter = 0;
    if let Ok(lines) = read_lines(file_path) {
        
        let mut line_counter: i32 = 0;
        for line in lines {
            if let Ok(val) = line {

                let mut vals: Vec<&str> = val.split(char::is_whitespace).collect();

                //println!("{:#}",vals);
                //println!("{:#}",vals[0]);
                let mut opponent = vals[0];
                //println!("{:#}",vals[1]);
                let mut player = vals[1];

                let score = get_star1_score(opponent.to_string(), player.to_string());

                outside_counter += score;
                
                line_counter+=1;
 
            }
        }
        println!("Processed {} lines ", line_counter);    
    }
    let final_output = outside_counter.to_string();

    return final_output.to_string();

}

fn get_game_results(opp:String, player:String) -> i8 {

    if opp == player {
        return 0;
    }
    if opp == "Rock" && player == "Scissors"{
        return -1;
    }
    if opp == "Scissors" && player == "Paper"{
        return -1;
    }
    if opp == "Paper" && player == "Rock" {
        return -1;
    }
    return 1;
}

fn get_star1_valmap(input:String)->String{
    if input == "A"{ return "Rock".to_string();}
    if input == "B"{ return "Paper".to_string();}
    if input == "C"{ return "Scissors".to_string();}
    if input == "X"{ return "Rock".to_string();}
    if input == "Y"{ return "Paper".to_string();}
    if input == "Z"{ return "Scissors".to_string();}
    return "".to_string();
}

fn get_star1_score(opp:String, player:String) -> i32 {

    // The score for a single round is the score for the 
    //   shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) 
    //   plus the score for the outcome of the round (0 if you lost, 3 if 
    //   the round was a draw, and 6 if you won).
    
    let mut totscore = 0;
    let oppstr = get_star1_valmap(opp);
    let plrstr = get_star1_valmap(player);

    // token score
    // rock 1
    // paper 2
    // sci 3
    totscore += if plrstr == "Rock" { 1 }else if plrstr == "Paper" { 2 }else { 3 };

    // game score
    // win 6
    // draw 3
    // lost 0
    let goutcome = get_game_results(oppstr, plrstr);
    totscore += if goutcome == 0 { 3 }else if goutcome > 0 { 6 }else {0};
    

    return totscore;
}

fn star2(file_path: String) -> String {
    println!("\n{} {}", function!(), file_path);
    
    let mut outside_counter = 0;
    if let Ok(lines) = read_lines(file_path) {
        
        let mut line_counter: i32 = 0;
        for line in lines {
            if let Ok(val) = line {

                let vals: Vec<&str> = val.split(char::is_whitespace).collect();

                //println!("{:#}",vals);
                //println!("{:#}",vals[0]);
                let opponent = vals[0];
                //println!("{:#}",vals[1]);
                let outcome = vals[1];

                let score = get_star1_score(opponent.to_string(), get_star2_playertoken(opponent.to_string(),outcome.to_string()));

                outside_counter += score;
                
                line_counter+=1;
 
            }
        }
        println!("Processed {} lines ", line_counter);    
    }
    let final_output = outside_counter.to_string();

    return final_output.to_string();
}

fn get_star2_playertoken(opp:String,outcome:String) -> String {

    // remap outcome:
    // X means you need to lose, 
    // Y means you need to end the round in a draw, 
    // and Z means you need to win.
    let oppstr = get_star1_valmap(opp.to_string());
    //A=Rock
    //B=Paper
    //C=Scissors
    if outcome == "Y" { return opp.to_string(); }
    if outcome == "Z" {
        // return winning hand
        if oppstr == "Rock" {
            return "B".to_string();
        }else if oppstr == "Paper" {
            return "C".to_string();
        }else { // assume Sci
            return "A".to_string();
        }
    }
    if outcome == "X" {
        // return losing hand
        if oppstr == "Rock" {
            return "C".to_string();
        }else if oppstr == "Paper" {
            return "A".to_string();
        }else { // assume Sci
            return "B".to_string();
        }
    }
    return "".to_string();
}