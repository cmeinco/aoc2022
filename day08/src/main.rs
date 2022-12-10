extern crate utils;
extern crate loggy;
use std::convert::TryInto;

use owo_colors::OwoColorize;
use regex::Regex;


struct Forest {
    trees: Vec<Vec<u8>>,
}

fn main() {
    log::set_logger(&loggy::Loggy {
        prefix: "AoC", // Typically, the name of the program.
        show_time: true, // Or false, if you prefer.
        show_thread: true, // Or false, if you prefer.
    }).unwrap();
    log::set_max_level(log::LevelFilter::Debug); // Or whatever level you want.

    loggy::info!("starting up");

    let mut expected_result = "21";
    utils::evaluate_results(star1("./input01-test.txt".to_string()), expected_result.to_string());
    utils::evaluate_results(star1("./input01-1.txt".to_string()), "-1".to_string()); // 13682
    expected_result = "8";
    utils::evaluate_results(star2("./input01-test.txt".to_string()), expected_result.to_string());
    utils::evaluate_results(star2("./input01-1.txt".to_string()), "-1".to_string()); // 12881
}

fn checkOpenViewLeft(f: &Vec<Vec<u32>>, tx: usize, ty:usize ) -> bool {
    for s in (0..tx).rev() {
        if f[tx][ty] <= f[s][ty] {
            return false;
        }
    }
    true
}

fn checkOpenViewRight(f: &Vec<Vec<u32>>, tx: usize, ty:usize ) -> bool {
    for s in (tx+1..f.len()) {
        if f[tx][ty] <= f[s][ty] {
            return false;
        }
    }
    true
}


fn checkOpenViewUp(f: &Vec<Vec<u32>>, tx: usize, ty:usize ) -> bool {
    for s in (0..ty).rev() {
        if f[tx][ty] <= f[tx][s] {
            return false;
        }
    }
    true
}


fn checkOpenViewDown(f: &Vec<Vec<u32>>, tx: usize, ty:usize ) -> bool {
    for s in (ty+1..f[0].len()) {
        if f[tx][ty] <= f[tx][s] {
            return false;
        }
    }
    true
}


#[loggy::scope]
fn getTreeHouseLocations(f: &Vec<Vec<u32>>) -> usize {

    println!("Processing {:?}", f);

    // add the edges
    let mut totSpots:usize = 2*f.len() + 2*(f[0].len()-2);

    // process everything inside the edges
    for x in 1..f.len()-1 {
        'nextree: for y in 1..f[0].len()-1 {
            let atree = f[x][y];

            println!("Checking {}@{},{}", atree, x, y);

            // from 0 to x; check all
            if checkOpenViewLeft(f,x,y){
                totSpots += 1;
                println!("{}@{},{} Open Left",f[x][y],x,y);
                continue 'nextree;
            }
            
            // from x to len()
            if checkOpenViewRight(f,x,y){
                totSpots += 1;
                println!("{}@{},{} Open Right",f[x][y],x,y);
                continue 'nextree;
            }

            // from 0 to y
            if checkOpenViewUp(f,x,y){
                totSpots += 1;
                println!("{}@{},{} Open Up",f[x][y],x,y);
                continue 'nextree;
            }

            // from y to len()
            if checkOpenViewDown(f,x,y){
                totSpots += 1;
                println!("{}@{},{} Open Down",f[x][y],x,y);
                continue 'nextree;
            }

           

        }
        println!(" ");
    }

    totSpots
}

#[loggy::scope]
fn star1(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);

    let mut forest: Vec<Vec<u32>> = Vec::new();
    // forest.push(vec![0i32, 1, 2, 3]);
    // forest.push(vec![1i32, 1, 2]);
    // println!("{:#?}", forest);


    let mut outside_counter = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        
        let mut line_counter: i32 = 0;
        for line in lines {
            if let Ok(val) = line {

                // println!("{}", val.green());

                // println!("{:?}",val.chars());
                //let chars = val.chars();
                let chars:Vec<char> = val.chars().collect();
                let mut digits:Vec<u32> = Vec::new();
                for c in &chars {
                    digits.push(c.to_digit(10).unwrap());
                }
                forest.push(digits);
                
                //forest.push(vec!(val.chars().map(|d| d.to_digit(10).unwrap()).collect()));
                println!("{:?} {:?}", chars, forest);

                // println!("{}", "strikethrough".strikethrough());
                // println!("My number is not {}!", 4.on_red());
                // println!("My number is {:#x}!", 10.green());


                line_counter+=1;
            }
        }
        loggy::debug!("Processed {} lines ", line_counter);   
        outside_counter=line_counter; 
    }

    let treeHouseLocations = getTreeHouseLocations(&forest);



    let final_output = treeHouseLocations.to_string();

    return final_output.to_string();

}


fn getScoreLeft(f: &Vec<Vec<u32>>, tx: usize, ty:usize ) -> usize {
    let mut treeCounter = 1;
    for s in (0..tx).rev() {
        if f[tx][ty] <= f[s][ty] {
            return treeCounter;
        }
        treeCounter+=1;
    }
    treeCounter-1
}

fn getScoreRight(f: &Vec<Vec<u32>>, tx: usize, ty:usize ) -> usize {
    let mut treeCounter = 1;
    for s in (tx+1..f.len()) {
        if f[tx][ty] <= f[s][ty] {
            return treeCounter;
        }
        treeCounter+=1;
    }
    treeCounter-1
}

fn getScoreUp(f: &Vec<Vec<u32>>, tx: usize, ty:usize ) -> usize {
    let mut treeCounter = 1;
    for s in (0..ty).rev() {
        if f[tx][ty] <= f[tx][s] {
            return treeCounter;
        }
        treeCounter+=1;
    }
    treeCounter-1
}

fn getScoreDown(f: &Vec<Vec<u32>>, tx: usize, ty:usize ) -> usize {
    let mut treeCounter = 1;
    for s in (ty+1..f[0].len()) {
        if f[tx][ty] <= f[tx][s] {
            return treeCounter;
        }
        treeCounter+=1;
    }
    treeCounter-1
}

#[loggy::scope]
fn getMaxTreeHouseScore(f: &Vec<Vec<u32>>) -> usize {

    println!("Processing {:?}", f);

    // add the edges
    let mut maxScore:usize = 0;

    // process everytree? lets start with middle, likely there anyway
    for x in 1..f.len()-1 {
        'nextree: for y in 1..f[0].len()-1 {
            let atree = f[x][y];

            println!("Checking {}@{},{}", atree, x, y);

            let mut runningScore:usize = 1;
            // from 0 to x; check all
            runningScore *= getScoreLeft(f, x, y);
            runningScore *= getScoreRight(f, x, y);
            runningScore *= getScoreUp(f, x, y);
            runningScore *= getScoreDown(f, x, y);
            

            if runningScore > maxScore {
                maxScore = runningScore;
            }
            

            

        }
        println!(" ");
    }

    maxScore
}

#[loggy::scope]
fn star2(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);

    let mut forest: Vec<Vec<u32>> = Vec::new();
    // forest.push(vec![0i32, 1, 2, 3]);
    // forest.push(vec![1i32, 1, 2]);
    // println!("{:#?}", forest);


    let mut outside_counter = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        
        let mut line_counter: i32 = 0;
        for line in lines {
            if let Ok(val) = line {

                // println!("{}", val.green());

                // println!("{:?}",val.chars());
                //let chars = val.chars();
                let chars:Vec<char> = val.chars().collect();
                let mut digits:Vec<u32> = Vec::new();
                for c in &chars {
                    digits.push(c.to_digit(10).unwrap());
                }
                forest.push(digits);
                
                //forest.push(vec!(val.chars().map(|d| d.to_digit(10).unwrap()).collect()));
                println!("{:?} {:?}", chars, forest);

                // println!("{}", "strikethrough".strikethrough());
                // println!("My number is not {}!", 4.on_red());
                // println!("My number is {:#x}!", 10.green());


                line_counter+=1;
            }
        }
        loggy::debug!("Processed {} lines ", line_counter);   
        outside_counter=line_counter; 
    }

    //let treeHouseLocations = getTreeHouseLocations(&forest);

    let highestScore:usize = getMaxTreeHouseScore(&forest);


    let final_output = highestScore.to_string();

    return final_output.to_string();
}
