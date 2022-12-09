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
    evaluate_results(day01_star1("./input01-test.txt".to_string()), "24000".to_string());
    evaluate_results(day01_star1("./input01-1.txt".to_string()), "-1".to_string()); // 71471
    evaluate_results(day01_star2("./input01-test.txt".to_string()), "45000".to_string());
    evaluate_results(day01_star2("./input01-1.txt".to_string()), "-1".to_string()); // 211189
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

fn day01_star1(file_path: String) -> String {
    println!("\n{} {}", function!(), file_path);
    
    let mut tot = 0;
    if let Ok(lines) = read_lines(file_path) {
        let mut tmp: i32 = 0;
        for line in lines {
            if let Ok(val) = line {
                if val == "" {
                    if tmp>tot {
                        tot = tmp;
                    }
                    tmp = 0;
                }else{
                    let ival= val.trim().parse::<i32>().expect("Wanted a Number");
                    tmp+=ival;
                }
            }
        }
    }
    let final_output = tot;

    return final_output.to_string();

}

fn day01_star2(file_path: String) -> String {
    println!("\n{} {}", function!(), file_path);

    let mut tot = 0;
    let mut top1 = 0;
    let mut top2 = 0;
    let mut top3 = 0;
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        let mut tmp: i32 = 0;
        for line in lines {
            if let Ok(val) = line {
                //println!("{}", val);
                if val == "" {
                    if tmp>tot {
                        tot = tmp;
                    }
                    if tmp>top1 {
                        // replace top 1
                        top3 = top2;
                        top2 = top1;
                        top1 = tmp;
                    }else if tmp>top2 {
                        // replace top 2
                        top3 = top2;
                        top2 = tmp;
                    }else if tmp>top3 {
                        // replace top 3
                        top3 = tmp;
                    }
                    tmp = 0;
                }else{
                    let ival= val.trim().parse::<i32>().expect("Wanted a Number");
                    tmp+=ival;
                }
            }
        }
    }
    
    let final_output = top1+top2+top3;
    return final_output.to_string();
}

// fn day01() {

//     println!("Day 01 *!");

//     let file_path = "./input01-1.txt";
//     //let file_path = "./input01-test.txt";
//     // 732302 too high

//     // --snip--
//     println!("In file {}", file_path);

//     //let contents = read_all_lines(&self.file_path);

//     let mut tot = 0;
//     let mut top1 = 0;
//     let mut top2 = 0;
//     let mut top3 = 0;
//     if let Ok(lines) = read_lines(file_path) {
//         // Consumes the iterator, returns an (Optional) String
//         let mut tmp: i32 = 0;
//         for line in lines {
//             if let Ok(val) = line {
//                 println!("{}", val);
//                 if val == "" {
//                     if tmp>tot {
//                         tot = tmp;
//                     }
//                     if tmp>top1 {
//                         // replace top 1
//                         top3 = top2;
//                         top2 = top1;
//                         top1 = tmp;
//                     }else if tmp>top2 {
//                         // replace top 2
//                         top3 = top2;
//                         top2 = tmp;
//                     }else if tmp>top3 {
//                         // replace top 3
//                         top3 = tmp;
//                     }
//                     tmp = 0;
//                 }else{
//                     let ival= val.trim().parse::<i32>().expect("Wanted a Number");
//                     tmp+=ival;
//                 }
//             }
//         }
//     }
//     // let contents = "something";
//     println!("Total:\n{}",tot);
//     println!("Top1 2 3: {} {} {}",top1,top2,top3);
//     final_output = (top1+top2+top3);
//     println!("TopSum {}",final_output);

// }

