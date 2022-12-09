use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use owo_colors::OwoColorize;
use loggy;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn evaluate_results(left: String, right: String) {

    // println!("{}", "strikethrough".strikethrough());
    // println!("My number is not {}!", 4.on_red());
    // println!("My number is {:#x}!", 10.green());
    if left != right {
        if right == "-1"{
            loggy::info!("[{}] {}", "FINAL OUTPUT".on_green(), left);
        }else{
            loggy::info!("[{}] DoesNotMatch {} != {} ", "FAIL".on_red(), left.red(), right.red());
        }
    }else{
        loggy::info!("[{}] {} ", "PASS".on_green(), left.green());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_eval_results_pass(){
        // how to read what is printed? need to return something; string? println!(eval_res?)
    }
}
