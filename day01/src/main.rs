use std::io::{self, BufRead};
use std::fs::File;
use std::iter::Skip;
use std::path::Path;

fn main() {
    day01();
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day01() {

    println!("Day 01 *!");

    let file_path = "./input01-1.txt";
    //let file_path = "./input01-test.txt";
    // 732302 too high

    // --snip--
    println!("In file {}", file_path);

    //let contents = read_all_lines(&self.file_path);

    let mut tot = 0;
    let mut top1 = 0;
    let mut top2 = 0;
    let mut top3 = 0;
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        let mut tmp: i32 = 0;
        for line in lines {
            if let Ok(val) = line {
                println!("{}", val);
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
    // let contents = "something";
    println!("Total:\n{}",tot);
    println!("Top1 2 3: {} {} {}",top1,top2,top3);
    println!("TopSum {}",(top1+top2+top3));
     
}