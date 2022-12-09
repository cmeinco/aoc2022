extern crate utils;
extern crate loggy;
use std::convert::TryFrom;

fn main() {
    log::set_logger(&loggy::Loggy {
        prefix: "AoC", // Typically, the name of the program.
        show_time: true, // Or false, if you prefer.
        show_thread: true, // Or false, if you prefer.
    }).unwrap();
    log::set_max_level(log::LevelFilter::Debug); // Or whatever level you want.

    loggy::info!("starting up");

    let mut expected_result = "CMZ";
    utils::evaluate_results(star1("./input01-test.txt".to_string()), expected_result.to_string());
    utils::evaluate_results(star1("./input01-1.txt".to_string()), "-1".to_string()); // 
    expected_result = "MCD";
    utils::evaluate_results(star2("./input01-test.txt".to_string()), expected_result.to_string());
    utils::evaluate_results(star2("./input01-1.txt".to_string()), "-1".to_string()); // 
}

#[loggy::scope]
fn star1(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);

    let mut stackvec:Vec<String> = Vec::with_capacity(10);
    let mut stacks_done = false;
    let mut movesteps:Vec<String> = Vec::with_capacity(20);

    let mut outside_counter = 0;

    if let Ok(lines) = utils::read_lines(file_path) {

        let mut line_counter: i32 = 0;
        for line in lines {
            if let Ok(val) = line {

                // read stacks
                // read until empty line
                if !stacks_done {
                    if val == "" {
                        stacks_done = true;
                    }else{
                        stackvec.push(val);
                    }
                }else{
                    // process inputs
                    movesteps.push(val);
                } 

                // read moves
                
                line_counter+=1;
            }
        }
        loggy::debug!("Processed {} lines ", line_counter);    
        loggy::debug!("StackLines {}; MoveSteps {}", stackvec.len(), movesteps.len());
    }

    let mut stacks:Vec<Vec<String>> = Vec::new();

    println!("{:?}", stackvec);
    println!("{}", stackvec[0].len()+1);
    for i in (0..(stackvec[0].len()+1)){
        stacks.push(Vec::new());
    }

    // parse the stacks vec
    for line in stackvec {
        println!("stack {} {}", line, line.len());

        // read each char
        // each 4 == a stack

        println!();
        let mut stack_id = 0;
        let mut c_ctr:i32=1;
        
        for c in line.chars() {

            if c != ' ' && c != '[' && c != ']' {
                print!("Add to stack:{};{},", stack_id,c);
                stacks[stack_id].push(c.to_string());
            }
            if c_ctr.rem_euclid(4) == 0 {
                stack_id += 1;
            }
            c_ctr += 1;
        }
        println!();

//     [D]    
// [N] [C]    
// [Z] [M] [P]
//  1   2   3 

//             [L] [M]         [M]    
//         [D] [R] [Z]         [C] [L]
//         [C] [S] [T] [G]     [V] [M]
// [R]     [L] [Q] [B] [B]     [D] [F]
// [H] [B] [G] [D] [Q] [Z]     [T] [J]
// [M] [J] [H] [M] [P] [S] [V] [L] [N]
// [P] [C] [N] [T] [S] [F] [R] [G] [Q]
// [Z] [P] [S] [F] [F] [T] [N] [P] [W]
//  1   2   3   4   5   6   7   8   9 

    }
    // get rid of the stack ids
    for s in (0..stacks.len()) {
        if stacks[s].len() > 0 {
            stacks[s].pop();
        }
    }
    loggy::debug!("{:?}",stacks);

    // do the moves (recursion?)
    for mv in movesteps {
        // parse moves 
        let mvvec:Vec<&str> = mv.split(" ").collect();
        let amt = mvvec[1].parse::<u8>().unwrap();;
        let stack_from = mvvec[3].parse::<usize>().unwrap() - 1;
        let stack_to = mvvec[5].parse::<usize>().unwrap() - 1;

        loggy::debug!("Action Move {} from {} to {}", amt, stack_from, stack_to);

        for i in (0..amt) {
            // move
            //stacks[stack_to][stacks[stack_to].len()-1]
            println!("{:?} -> {:?}",stacks[stack_from],stacks[stack_to]);

            stacks[stack_from].rotate_left(1);
            //let mut val = stacks[stack_from].shift();
            let val = stacks[stack_from].pop().expect("Something here.");
            stacks[stack_to].push(val);
            stacks[stack_to].rotate_right(1);

            println!("{:?} -> {:?}",stacks[stack_from],stacks[stack_to]);
            //let val = stacks[stack_from].pop().expect("something");
            //stacks[stack_to].push(val);
            println!("{} {}", stack_from, stack_to);
        }
        println!("{:?}", stacks);
    }

    let mut final_output = "".to_string();
    for s in (0..stacks.len()) {
        if stacks[s].len() > 0 {
            let v = stacks[s][0].to_string();
            final_output.push_str(&v.to_string());
        }
    } 

    //let final_output = outside_counter.to_string();

    return final_output.to_string();

}


#[loggy::scope]
fn star2(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);

    let mut stackvec:Vec<String> = Vec::with_capacity(10);
    let mut stacks_done = false;
    let mut movesteps:Vec<String> = Vec::with_capacity(20);

    let mut outside_counter = 0;

    if let Ok(lines) = utils::read_lines(file_path) {

        let mut line_counter: i32 = 0;
        for line in lines {
            if let Ok(val) = line {

                // read stacks
                // read until empty line
                if !stacks_done {
                    if val == "" {
                        stacks_done = true;
                    }else{
                        stackvec.push(val);
                    }
                }else{
                    // process inputs
                    movesteps.push(val);
                } 

                // read moves
                
                line_counter+=1;
            }
        }
        loggy::debug!("Processed {} lines ", line_counter);    
        loggy::debug!("StackLines {}; MoveSteps {}", stackvec.len(), movesteps.len());
    }

    let mut stacks:Vec<Vec<String>> = Vec::new();

    println!("{:?}", stackvec);
    println!("{}", stackvec[0].len()+1);
    for i in (0..(stackvec[0].len()+1)){
        stacks.push(Vec::new());
    }

    // parse the stacks vec
    for line in stackvec {
        println!("stack {} {}", line, line.len());

        // read each char
        // each 4 == a stack

        println!();
        let mut stack_id = 0;
        let mut c_ctr:i32=1;
        
        for c in line.chars() {

            if c != ' ' && c != '[' && c != ']' {
                print!("Add to stack:{};{},", stack_id,c);
                stacks[stack_id].push(c.to_string());
            }
            if c_ctr.rem_euclid(4) == 0 {
                stack_id += 1;
            }
            c_ctr += 1;
        }
        println!();

//     [D]    
// [N] [C]    
// [Z] [M] [P]
//  1   2   3 

//             [L] [M]         [M]    
//         [D] [R] [Z]         [C] [L]
//         [C] [S] [T] [G]     [V] [M]
// [R]     [L] [Q] [B] [B]     [D] [F]
// [H] [B] [G] [D] [Q] [Z]     [T] [J]
// [M] [J] [H] [M] [P] [S] [V] [L] [N]
// [P] [C] [N] [T] [S] [F] [R] [G] [Q]
// [Z] [P] [S] [F] [F] [T] [N] [P] [W]
//  1   2   3   4   5   6   7   8   9 

    }
    // get rid of the stack ids
    for s in (0..stacks.len()) {
        if stacks[s].len() > 0 {
            stacks[s].pop();
        }
    }
    loggy::debug!("{:?}",stacks);

    // do the moves (recursion?)
    for mv in movesteps {
        // parse moves 
        let mvvec:Vec<&str> = mv.split(" ").collect();
        let amt = mvvec[1].parse::<u8>().unwrap();;
        let stack_from = mvvec[3].parse::<usize>().unwrap() - 1;
        let stack_to = mvvec[5].parse::<usize>().unwrap() - 1;

        loggy::debug!("Action Move {} from {} to {}", amt, stack_from, stack_to);

        for i in (0..amt) {
            // move
            //stacks[stack_to][stacks[stack_to].len()-1]
            println!("{:?} -> {:?}",stacks[stack_from],stacks[stack_to]);

            stacks[stack_from].rotate_left(1);
            //let mut val = stacks[stack_from].shift();
            let val = stacks[stack_from].pop().expect("Something here.");
            stacks[stack_to].push(val);
            //stacks[stack_to].rotate_right(1);

            println!("{:?} -> {:?}",stacks[stack_from],stacks[stack_to]);
            //let val = stacks[stack_from].pop().expect("something");
            //stacks[stack_to].push(val);
            println!("{} {}", stack_from, stack_to);
        }
        for i in (0..amt) {
            stacks[stack_to].rotate_right(1);
        }
        println!("{:?}", stacks);
    }

    let mut final_output = "".to_string();
    for s in (0..stacks.len()) {
        if stacks[s].len() > 0 {
            let v = stacks[s][0].to_string();
            final_output.push_str(&v.to_string());
        }
    } 

    //let final_output = outside_counter.to_string();

    return final_output.to_string();

}

