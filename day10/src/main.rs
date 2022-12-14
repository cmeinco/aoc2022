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

    let mut expected_result = "13140";
    utils::evaluate_results(star1("./input01-test.txt".to_string()), expected_result.to_string());
    utils::evaluate_results(star1("./input01-1.txt".to_string()), "-1".to_string()); // 13682
    expected_result = "12";
    utils::evaluate_results(star2("./input01-test.txt".to_string()), expected_result.to_string());
    utils::evaluate_results(star2("./input01-1.txt".to_string()), "-1".to_string()); // 12881
}

#[loggy::scope]
fn star1(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);

    let mut cycle_ctr:usize = 0;
    let mut register_x:i32 = 1;


    let mut outside_counter:i32 = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        
        let mut line_counter: i32 = 0;
        for line in lines {
            if let Ok(val) = line {

                //println!("{}", val.green());

                let input_cmd:Vec<&str> = val.split(" ").collect();
                let cmd:&str = input_cmd[0];



                if val == "noop" {
                    cycle_ctr += 1;

                    // 20, 60, 100, 140, 180, 220
                    if cycle_ctr == 20 { println!("20 C:{} X:{} S:{}",cycle_ctr,register_x,(20*register_x)); outside_counter+=(20*register_x);}
                    if cycle_ctr == 60 { println!("60 C:{} X:{} S:{}",cycle_ctr,register_x,(60*register_x)); outside_counter+=(60*register_x);}
                    if cycle_ctr == 100 {  println!("100 C:{} X:{} S:{}",cycle_ctr,register_x,(100*register_x)); outside_counter+=(100*register_x);}
                    //if cycle_ctr == 120 { println!("120 C:{} X:{} S:{}",cycle_ctr,register_x,(120*register_x));  outside_counter+=(120*register_x);}
                    if cycle_ctr == 140 {  println!("140 C:{} X:{} S:{}",cycle_ctr,register_x,(140*register_x)); outside_counter+=(140*register_x);}
                    if cycle_ctr == 180 {  println!("180 C:{} X:{} S:{}",cycle_ctr,register_x,(180*register_x)); outside_counter+=(180*register_x);}
                    if cycle_ctr == 220 {  println!("220 C:{} X:{} S:{}",cycle_ctr,register_x,(220*register_x)); outside_counter+=(220*register_x);}


                }else if cmd == "addx" {
                                
                    cycle_ctr += 1;
                                                    
                    // 20, 60, 100, 140, 180, 220
                    if cycle_ctr == 20 { println!("20 C:{} X:{} S:{}",cycle_ctr,register_x,(20*register_x)); outside_counter+=(20*register_x);}
                    if cycle_ctr == 60 { println!("60 C:{} X:{} S:{}",cycle_ctr,register_x,(60*register_x)); outside_counter+=(60*register_x);}
                    if cycle_ctr == 100 {  println!("100 C:{} X:{} S:{}",cycle_ctr,register_x,(100*register_x)); outside_counter+=(100*register_x);}
                    //if cycle_ctr == 120 { println!("120 C:{} X:{} S:{}",cycle_ctr,register_x,(120*register_x));  outside_counter+=(120*register_x);}
                    if cycle_ctr == 140 {  println!("140 C:{} X:{} S:{}",cycle_ctr,register_x,(140*register_x)); outside_counter+=(140*register_x);}
                    if cycle_ctr == 180 {  println!("180 C:{} X:{} S:{}",cycle_ctr,register_x,(180*register_x)); outside_counter+=(180*register_x);}
                    if cycle_ctr == 220 {  println!("220 C:{} X:{} S:{}",cycle_ctr,register_x,(220*register_x)); outside_counter+=(220*register_x);}

                    cycle_ctr += 1;
                   
                    // 20, 60, 100, 140, 180, 220
                    if cycle_ctr == 20 { println!("20 C:{} X:{} S:{}",cycle_ctr,register_x,(20*register_x)); outside_counter+=(20*register_x);}
                    if cycle_ctr == 60 { println!("60 C:{} X:{} S:{}",cycle_ctr,register_x,(60*register_x)); outside_counter+=(60*register_x);}
                    if cycle_ctr == 100 {  println!("100 C:{} X:{} S:{}",cycle_ctr,register_x,(100*register_x)); outside_counter+=(100*register_x);}
                    //if cycle_ctr == 120 { println!("120 C:{} X:{} S:{}",cycle_ctr,register_x,(120*register_x));  outside_counter+=(120*register_x);}
                    if cycle_ctr == 140 {  println!("140 C:{} X:{} S:{}",cycle_ctr,register_x,(140*register_x)); outside_counter+=(140*register_x);}
                    if cycle_ctr == 180 {  println!("180 C:{} X:{} S:{}",cycle_ctr,register_x,(180*register_x)); outside_counter+=(180*register_x);}
                    if cycle_ctr == 220 {  println!("220 C:{} X:{} S:{}",cycle_ctr,register_x,(220*register_x)); outside_counter+=(220*register_x);}

                    let amt:i32 = input_cmd[1].parse::<i32>().unwrap();
                    register_x+=amt;

                }
                loggy::debug!("Cycle: {} [{}]", cycle_ctr.green(),register_x.green());


                line_counter+=1;
            }
        }
        loggy::debug!("Processed {} lines ", line_counter);   
        //outside_counter=line_counter; 
    }
    let final_output = outside_counter.to_string();

    return final_output.to_string();

}

#[loggy::scope]
fn star2(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);

    let mut cmds:Vec<String> = Vec::with_capacity(100);
    let mut cmds_modified:Vec<String> = Vec::with_capacity(100);
    // let mut stackvec:Vec<String> = Vec::with_capacity(10);

    let mut outside_counter:i32 = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        
        let mut line_counter: i32 = 0;
        for line in lines {
            if let Ok(val) = line {

                //println!("{}", val.green());

                cmds.push(val);

                line_counter+=1;
            }
        }
        println!("Loaded {} lines.", line_counter);

        for c in &cmds {
            let input_cmd:Vec<&str> = c.split(" ").collect();
            let cmd = input_cmd[0];
            //let cmd_val = input_cmd[1].parse::<i32>().unwrap();

            if cmd == "noop"{
                cmds_modified.push(String::from("noop"));
            }else{
                // insert noop, noop, cmd
                //cmds_modified.push(String::from("noop"));
                cmds_modified.push(String::from("noop"));
                cmds_modified.push(String::from(c));
            }
        }


        let mut cycle_ctr:i32 = 1;
        let mut register_x:i32 = 1;
        let mut ptr:usize = 0;
        let mut horiz_ptr:i32 = 0;


        // { // priming read
        //     let input_cmd:Vec<&str> = cmds[0].split(" ").collect();
        //     cur_cmd = input_cmd[0];
        //     cur_cmd_val = input_cmd[1].parse::<i32>().unwrap();
        //     if cur_cmd == "noop" {
        //         cmd_exec = true;
        //         cur_cmd_ctr = 0;
        //     }else{
        //         cmd_exec = false;
        //         cur_cmd_ctr = 2;
        //     }
        // }

        println!("{:?}",cmds_modified);

        loop {
            
            // loop for each cycle / tick
            //if ptr == cmds.len(){println!("RUT ROH.");break;}
            //println!("P{},cmd{}",ptr,cmds[ptr]);
            let neighbor:i32 = horiz_ptr-1;
            let neighbor2:i32 = horiz_ptr+1;

            if register_x == neighbor || register_x == horiz_ptr || register_x == neighbor2 {
                //print!("# {}",register_x);
                print!("#");
            }else{
                //print!(". {}", register_x);
                print!(".")
            }

            if cmds_modified[ptr] == "noop" {

            }else{
                let input_cmd:Vec<&str> = cmds_modified[ptr].split(" ").collect();
                //next_cmd = input_cmd[0];
                let next_cmd_val:i32 = input_cmd[1].parse::<i32>().unwrap();
                register_x += next_cmd_val;
            }

            if horiz_ptr == 40 { println!("");horiz_ptr=0;}
            // if cycle_ctr == 80 { println!("");horiz_ptr=0;}
            // if cycle_ctr == 120 { println!("");horiz_ptr=0;}
            // if cycle_ctr == 160 { println!("");horiz_ptr=0;}
            // if cycle_ctr == 200 { println!("");horiz_ptr=0;}
            if cycle_ctr == 240 { println!("");horiz_ptr=0;break;}
            cycle_ctr +=1;
            horiz_ptr +=1;
            ptr+=1;
        }
    
        //loggy::debug!("Processed {} lines ", line_counter);   
        //outside_counter=line_counter; 
    }
    let final_output = outside_counter.to_string();

    return final_output.to_string();

}
