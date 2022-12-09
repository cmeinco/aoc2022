extern crate utils;
extern crate loggy;
use owo_colors::OwoColorize;

fn main() {
    log::set_logger(&loggy::Loggy {
        prefix: "AoC", // Typically, the name of the program.
        show_time: true, // Or false, if you prefer.
        show_thread: true, // Or false, if you prefer.
    }).unwrap();
    log::set_max_level(log::LevelFilter::Debug); // Or whatever level you want.

    loggy::info!("starting up");

    let mut expected_result = "15";
    utils::evaluate_results(star1("./input01-test.txt".to_string()), expected_result.to_string());
    utils::evaluate_results(star1("./input01-1.txt".to_string()), "-1".to_string()); // 13682
    expected_result = "12";
    utils::evaluate_results(star2("./input01-test.txt".to_string()), expected_result.to_string());
    utils::evaluate_results(star2("./input01-1.txt".to_string()), "-1".to_string()); // 12881
}

#[loggy::scope]
fn star1(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);

    let mut outside_counter = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        
        let mut line_counter: i32 = 0;
        for line in lines {
            if let Ok(val) = line {

                println!("{}", val.green());

                // println!("{}", "strikethrough".strikethrough());
                // println!("My number is not {}!", 4.on_red());
                // println!("My number is {:#x}!", 10.green());

                line_counter+=1;
            }
        }
        loggy::debug!("Processed {} lines ", line_counter);   
        outside_counter=line_counter; 
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
             
                println!("{}", val);

                line_counter+=1;
 
            }
        }
        loggy::debug!("Processed {} lines ", line_counter); 
        outside_counter=line_counter;   
    }
    let final_output = outside_counter.to_string();

    return final_output.to_string();
}
