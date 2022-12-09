extern crate utils;
extern crate loggy;

fn main() {
    log::set_logger(&loggy::Loggy {
        prefix: "AoC", // Typically, the name of the program.
        show_time: true, // Or false, if you prefer.
        show_thread: true, // Or false, if you prefer.
    }).unwrap();
    log::set_max_level(log::LevelFilter::Debug); // Or whatever level you want.

    loggy::info!("starting up");

    let mut expected_result = "7";
    utils::evaluate_results(star1("./input01-test.txt".to_string()), expected_result.to_string());
    expected_result = "5";
    utils::evaluate_results(star1("./input01-test2.txt".to_string()), expected_result.to_string());
    expected_result = "6";
    utils::evaluate_results(star1("./input01-test3.txt".to_string()), expected_result.to_string());
    expected_result = "10";
    utils::evaluate_results(star1("./input01-test4.txt".to_string()), expected_result.to_string());
    expected_result = "11";
    utils::evaluate_results(star1("./input01-test5.txt".to_string()), expected_result.to_string());
    
    utils::evaluate_results(star1("./input01-1.txt".to_string()), "-1".to_string()); // 13682


    expected_result = "19";
    utils::evaluate_results(star2("./input01-test.txt".to_string()), expected_result.to_string());
    expected_result = "23";
    utils::evaluate_results(star2("./input01-test2.txt".to_string()), expected_result.to_string());
    expected_result = "23";
    utils::evaluate_results(star2("./input01-test3.txt".to_string()), expected_result.to_string());
    expected_result = "29";
    utils::evaluate_results(star2("./input01-test4.txt".to_string()), expected_result.to_string());
    expected_result = "26";
    utils::evaluate_results(star2("./input01-test5.txt".to_string()), expected_result.to_string());
    
    utils::evaluate_results(star2("./input01-1.txt".to_string()), "-1".to_string()); // 12881
}

#[loggy::scope]
fn star1(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);

    let mut outside_counter = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        
        let mut line_counter: i32 = 0;
        'endit: for line in lines {
            if let Ok(val) = line {

                let vals:Vec<char> = val.chars().collect();

                let window_size = 4;

                'outer: for i in (0..vals.len()) {
                    // read window i to i+3

                    for j in i..i+window_size-1 {
                        for k in j+1..i+window_size {
                            if vals[j] == vals[k] { continue 'outer; } 
                        }
                    }                           
                    outside_counter = i+3+1;
                    continue 'endit;

                }

                line_counter+=1;
            }
        }
        loggy::debug!("Processed {} lines ", line_counter);    
    }
    let final_output = outside_counter.to_string();

    return final_output.to_string();

}


#[loggy::scope]
fn star1_old(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);

    let mut outside_counter = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        
        let mut line_counter: i32 = 0;
        'endit: for line in lines {
            if let Ok(val) = line {

                let vals:Vec<char> = val.chars().collect();

                'outer: for i in (0..vals.len()) {
                    // read window i to i+3
                    for j in (i..i+3){
                        // are any of the chars the same?
                        if vals[j] == vals[j+1] { continue 'outer; }
                        if vals[j] == vals[j+2] { continue 'outer; }
                        if vals[j] == vals[j+3] { continue 'outer; }
                        
                        if vals[j+1] == vals[j+2] { continue 'outer; }
                        if vals[j+1] == vals[j+3] { continue 'outer; }

                        if vals[j+2] == vals[j+3] { continue 'outer; }
                        
                        outside_counter = j+3+1;
                        continue 'endit;
                    }

                }

                line_counter+=1;
            }
        }
        loggy::debug!("Processed {} lines ", line_counter);    
    }
    let final_output = outside_counter.to_string();

    return final_output.to_string();

}

#[loggy::scope]
fn star2(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);

    let mut outside_counter = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        
        let mut line_counter: i32 = 0;
        'endit: for line in lines {
            if let Ok(val) = line {

                let vals:Vec<char> = val.chars().collect();

                let window_size = 14;

                'outer: for i in (0..vals.len()) {
                    // read window i to i+3

                    for j in i..i+window_size-1 {
                        for k in j+1..i+window_size {
                            if vals[j] == vals[k] { continue 'outer; } 
                        }
                    }                           
                    outside_counter = i+window_size;
                    continue 'endit;

                }

                line_counter+=1;
            }
        }
        loggy::debug!("Processed {} lines ", line_counter);    
    }
    let final_output = outside_counter.to_string();

    return final_output.to_string();
}
