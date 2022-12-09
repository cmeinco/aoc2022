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

    let mut expected_result = "2";
    utils::evaluate_results(star1("./input01-test.txt".to_string()), expected_result.to_string());
    utils::evaluate_results(star1("./input01-1.txt".to_string()), "-1".to_string()); // 471
    expected_result = "4";
    utils::evaluate_results(star2("./input01-test.txt".to_string()), expected_result.to_string());
    utils::evaluate_results(star2("./input01-1.txt".to_string()), "-1".to_string()); // 942-High; // 888
}

#[loggy::scope]
fn star1(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);

    let mut outside_counter = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        
        let mut line_counter: i32 = 0;
        for line in lines {
            if let Ok(val) = line {

                let vals: Vec<&str> = val.split(",").collect();
                let left = vals[0];
                let lefts:Vec<&str> = left.split("-").collect();
                let left_min = lefts[0].parse::<i32>().unwrap();
                let left_max = lefts[1].parse::<i32>().unwrap();

                let right = vals[1];
                let rights:Vec<&str> = right.split("-").collect();
                let right_min = rights[0].parse::<i32>().unwrap();
                let right_max = rights[1].parse::<i32>().unwrap();

                loggy::info!("left {} <min:{},max:{}> right {} <min:{},max:{}>", left, left_min, left_max, right, right_min, right_max);
                //let score = get_star1_score(opponent.to_string(), player.to_string());

                if left_min>=right_min && left_max <= right_max {
                    // left is inside right
                    outside_counter+=1;
                    //loggy::info!("Found Overlap {}", val);
                }else if right_min>=left_min && right_max <= left_max {
                    // right is inside left
                    outside_counter+=1;
                    //loggy::info!("Found Overlap {}", val);
                }
                // outside_counter += score;
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
        for line in lines {
            if let Ok(val) = line {

                let vals: Vec<&str> = val.split(",").collect();
                let left = vals[0];
                let lefts:Vec<&str> = left.split("-").collect();
                let left_min = lefts[0].parse::<i32>().unwrap();
                let left_max = lefts[1].parse::<i32>().unwrap();

                let right = vals[1];
                let rights:Vec<&str> = right.split("-").collect();
                let right_min = rights[0].parse::<i32>().unwrap();
                let right_max = rights[1].parse::<i32>().unwrap();

                //loggy::info!("left {} <min:{},max:{}> right {} <min:{},max:{}>", left, left_min, left_max, right, right_min, right_max);
                //let score = get_star1_score(opponent.to_string(), player.to_string());

                if left_min==right_min || left_max==right_min || left_min==right_max || left_max==right_max {
                    outside_counter+=1;
                    //loggy::info!("Found Overlap {}", val);
                }else if left_min>=right_min && left_min <= right_max {
                    // left 
                    outside_counter+=1;
                    //loggy::info!("Found Overlap {}", val);
                }else if left_max>=right_min && left_max <= right_max {
                    // right is inside left
                    outside_counter+=1;
                    //loggy::info!("Found Overlap {}", val);
                }else if right_min>=left_min && right_min <= left_max {
                    // left 
                    outside_counter+=1;
                    //loggy::info!("Found Overlap {}", val);
                }else if right_max>=left_min && right_max <= left_max {
                    // right is inside left
                    outside_counter+=1;
                    //loggy::info!("Found Overlap {}", val);
                }
                // outside_counter += score;
                line_counter+=1;
 
            }
        }
        loggy::debug!("Processed {} lines ", line_counter);    
    }
    let final_output = outside_counter.to_string();

    return final_output.to_string();
}
