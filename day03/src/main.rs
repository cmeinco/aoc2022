use std::collections::HashMap;

extern crate utils;

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
    utils::evaluate_results(star1("./input01-test.txt".to_string()), "157".to_string());
    utils::evaluate_results(star1("./input01-1.txt".to_string()), "-1".to_string()); // 948-Low; 7795
    utils::evaluate_results(star2("./input01-test.txt".to_string()), "70".to_string());
    utils::evaluate_results(star2("./input01-1.txt".to_string()), "-1".to_string()); // 857-Low;
}

fn get_priority(item:String) -> i32 {

    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.




    return 0;
}

fn star1(file_path: String) -> String {
    println!("\n{} {}", function!(), file_path);

    let mut dupes:Vec<String> = Vec::with_capacity(5);

    let mut outside_counter = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        
        let mut line_counter: i32 = 0;
        for line in lines {
            if let Ok(val) = line {

                let mut linedupe:Vec<String> = Vec::with_capacity(5);

                //let mut vals: Vec<&str> = val.split("").collect();

                //println!("{:#?}",vals);
                
                //println!("Length {} ", val.chars().count());

                let (half1,half2) = val.split_at(val.chars().count()/2);
                //println!("{} {}", half1, half2);
                // split the string in half

                for i in half1.chars() {
                    for j in half2.chars() {
                        if i == j {
                            // if not in dupes push
                            if !linedupe.contains(&i.to_string()) {
                                dupes.push(i.to_string());
                                linedupe.push(i.to_string());
                            }
                            
                            //println!("Dupe Found {} ", i);
                        }
                    }
                }
                
                line_counter+=1;
 
            }
        }
        println!("Processed {} lines ", line_counter);    
    }

    // for each duplicate item sum up the priorities
    //println!("{:#?}",dupes);
    
    let mut final_output = 0;// outside_counter.to_string();

    let mut lookupmap = HashMap::new();

    let mut ctr = 1;
    for c in b'a'..=b'z' {
        //print!("{} ", c as char);
        lookupmap.insert(String::from(c as char), ctr);
        ctr += 1;
    }
    for c in b'A'..=b'Z' {
        //print!("{} ", c as char);
        lookupmap.insert(String::from(c as char), ctr);
        ctr += 1;
    }
    //println!("{:#?}",lookupmap);

    for d in dupes {
        //println!("Looking Up {} ", d);
        final_output += lookupmap.get(&d).unwrap();
    }
    println!("");

    return final_output.to_string();

}

// regex ^.*?(.).*? .*?\1.*? .*?\1.*?
fn star2(file_path: String) -> String {
    println!("\n{} {}", function!(), file_path);

    let mut dupes:Vec<String> = Vec::with_capacity(5);

    let mut outside_counter = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        
        let mut line_counter: i32 = 1;

        //let mut linedupe:Vec<String> = Vec::with_capacity(5);

        let mut line1: String = "".to_string();
        let mut line2: String = "".to_string();
        let mut line3: String = "".to_string();

        for line in lines {
            if let Ok(val) = line {

                if line_counter.rem_euclid(3) == 0 {
                    line3 = val.to_string();
                    //println!("Set Line3 {}", line3);
                    //linedupe = Vec::with_capacity(5);

                    //println!("[{}] Comparing {}; {}; {}", line_counter, line1, line2, line3);

                    'outer: for i in line1.chars() {
                        for j in line2.chars() {
                            for k in line3.chars() {
                                if i == j && i == k {
                                    // if not in dupes push
                                    //if !dupes.contains(&i.to_string()) {
                                    dupes.push(i.to_string());
                                        //linedupe.push(i.to_string());
                                    println!("Breaking on {}", i);
                                    break 'outer;
                                    //}
                                    
                                    //println!("Dupe Found {} ", i);
                                }
                            }
                        }
                    }
                    println!("[{}] No Dupe Found {} {} {}", line_counter, line1, line2, line3);
    
                }else if line_counter.rem_euclid(3) == 1 {
                    line1 = val.to_string();
                    //println!("Set Line1 {}", line1);

                }else if line_counter.rem_euclid(3) == 2 {
                    line2 = val.to_string();
                    //println!("Set Line2 {}", line2);
                }
                line_counter+=1;
            }
        }
        println!("Processed {} lines ", line_counter);    
    }

    // for each duplicate item sum up the priorities
    //println!("DUPES: {:#?}",dupes);
    
    let mut final_output = 0;// outside_counter.to_string();

    let mut lookupmap = HashMap::new();

    let mut ctr = 1;
    for c in b'a'..=b'z' {
        //print!("{} ", c as char);
        lookupmap.insert(String::from(c as char), ctr);
        ctr += 1;
    }
    for c in b'A'..=b'Z' {
        //print!("{} ", c as char);
        lookupmap.insert(String::from(c as char), ctr);
        ctr += 1;
    }
    //println!("{:#?}",lookupmap);

    for d in dupes {
        //println!("Looking Up {} ", d);
        final_output += lookupmap.get(&d).unwrap();
    }
    println!("");

    return final_output.to_string();

}

