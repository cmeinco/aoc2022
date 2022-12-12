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

    let mut expected_result = "13";
    utils::evaluate_results(star1("./input01-test.txt".to_string()), expected_result.to_string());
    utils::evaluate_results(star1("./input01-1.txt".to_string()), "-1".to_string()); // 233-Low, 5051-Low, 6072-Low
    expected_result = "1";
    utils::evaluate_results(star2("./input01-test.txt".to_string()), expected_result.to_string());
    expected_result = "36";
    utils::evaluate_results(star2("./input01-test2.txt".to_string()), expected_result.to_string());
    
    utils::evaluate_results(star2("./input01-1.txt".to_string()), "-1".to_string()); // 1232-Low; 2493
}

// fn checkAndCorrectTail(hx:i32,hy:i32,mut tx:i32,mut ty:i32, mut poopSpots:&Vec<String>) {
//     ty+=1;
//     let key:String = format!("{}_{}",tx,ty);
//     if !poopSpots.contains(&key) {
//         println!("Poop!");
//         poopSpots.push(key);
//     }
// }

// fn moveHeadRight(mut hx:i32,mut hy:i32,mut tx:i32,mut ty:i32, mut poopSpots:&Vec<String>) {
//     hx += 1;
//     checkAndCorrectTail(hx,hy,tx,ty,&poopSpots);
// }

fn correctTail(hx:i32,hy:i32,mut tx:i32,mut ty:i32) -> (i32,i32) {

    if hx == tx && hy == ty { /* same spot, do nothing */}
    else if hx == tx { 
        // println!("x equal"); 
        // println!("H: {},{} T: {},{} ",hx,hy,tx,ty);
        // println!("sub;{};abs;{}",hy-ty,hy.abs()-ty.abs());
        if (hy.abs()-ty.abs()).abs() > 1 {
            //println!("y off by 2; adjusting.");
            if hy-ty > 0 {
                ty += 1;
            }else{
                ty -= 1;
            }

        }else{
            //println!("Y Appears to still be a neighbor.")
        }
    }
    else if hy == ty { 
        // println!("y equal"); 
        // println!("H: {},{} T: {},{} ",hx,hy,tx,ty);
        // println!("sub;{};abs;{}",hx-tx,hx.abs()-tx.abs());
        if (hx.abs()-tx.abs()).abs() > 1 {
            //println!("x off by 2; adjusting.");
            if hx-tx > 0 {
                tx += 1;
            }else{
                tx -= 1;
            }
        }else{
            //println!("X Appears to still be a neighbor.")
        }
    }
    else { 
        // println!("xy not equal on the diag"); 

        //println!("ON DIAG: H: {},{} T: {},{} ",hx,hy,tx,ty);
        // println!("sub;{};abs;{}",hx-tx,hx.abs()-tx.abs());

        if (hx.abs() - tx.abs()).abs() < 2 && (hy.abs() - ty.abs()).abs() < 2 {
            //println!("appears to be neighbor, no change.");

        }else if (hx.abs() - tx.abs()).abs() < 2 {
            // y off by 2
            //println!("y off by 2; adjusting.");

            // if hx > tx {
            //     tx +=1;
            // }else{
            //     tx -=1;
            // }
            // if hy > ty {
            //     ty +=1;
            // }else {
            //     ty -=1;
            // }
            if hy-ty > 0 {
                ty += 1;
            }else{
                ty -= 1;
            }
            if hx-tx > 0 {
                tx += 1;
            }else{
                tx -= 1;
            }
        }else if (hy.abs() - ty.abs()).abs() < 2 {
            // x off by 2
            //println!("x off by 2; adjusting. H: {},{} T: {},{} ",hx,hy,tx,ty);

            // if hx > tx {
            //     tx +=1;
            // }else{
            //     tx -=1;
            // }
            // if hy > ty {
            //     ty +=1;
            // }else {
            //     ty -=1;
            // }
            if hy-ty > 0 {
                ty += 1;
            }else{
                ty -= 1;
            }
            if hx-tx > 0 {
                tx += 1;
            }else{
                tx -= 1;
            }
            //println!("x off by 2; adjusting DONE. H: {},{} T: {},{} ",hx,hy,tx,ty);
        }else{
            println!("Dont know this case ====== H: {},{} T: {},{} ",hx,hy,tx,ty);
            //panic!("Rut Roh");

            // when a tail double jumps need to move in the dir of less by 1.
            // move diag towards the head
            if hy-ty > 0 {
                ty += 1;
            }else{
                ty -= 1;
            }
            if hx-tx > 0 {
                tx += 1;
            }else{
                tx -= 1;
            }
        }
    }

    (tx,ty)
}

#[loggy::scope]
fn star1(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);

    let mut poopzone:Vec<String> = Vec::new();

    let mut hx:i32 = 10000;
    let mut hy:i32 = 10000;
    let mut tx:i32 = 10000;
    let mut ty:i32 = 10000;
    let mut sx:i32 = 0;
    let mut sy:i32 = 0;

    let mut outside_counter = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        
        let mut line_counter: i32 = 0;
        'nextline: for line in lines {
            if let Ok(val) = line {
                line_counter+=1;
                //loggy::debug!("=== BEGIN STEP {}: H:{},{} T:{},{}", line_counter, hx,hy, tx,ty);

                let mut loop_counter:i32 = 4;

                // parse direction and actionctr
                let mut actions:Vec<&str> = val.split(" ").collect();
                //println!("{:?}", actions);
                let action = actions[0];
                let mut action_ctr:i32 = actions[1].parse::<i32>().unwrap();

                loop {
                    loop_counter+=1;action_ctr-=1;

                    // loggy::debug!("Loop {}: Action: {} CTR: {} Line:{}", loop_counter, action, action_ctr, val.green());
                    // loggy::debug!("Pre Move: T:{},{} H: {},{}",tx,ty, hx,hy);

                    match action {
                        "R" => { // move right 1
                            //loggy::debug!("Going Right.");
                            hx += 1;
                        }
                        "L" => {  // move left 1
                            //loggy::debug!("Going Left.");
                            hx -= 1;
                        }
                        "U" => { // move up 1
                            //loggy::debug!("Going Up.");
                            hy += 1;
                        }
                        "D" => { // move down 1
                            //loggy::debug!("Going Down.");
                            hy -= 1;
                        }
                        _ => {panic!("Invalid Action {}", action);}

                    }
                    /*
                    ..T#..
                    ..H##.
                    .####.
                    ....#.
                    s###..
                    H: 1,-3 T:2,-4
                    */
                    // correct the tail location
                    {
                        (tx,ty) = correctTail(hx,hy,tx,ty);
                        // // if tail more than 1 away from head move closer
                        // if hx == tx && hy == ty { println!("Do Nota");/* in same loc, do nothing */}
                        // else if hx == tx { /* in same row, check col */
                        //     loggy::debug!("Expecting Col Move. PreMove T:{},{}; H: {},{}",tx,ty, hx,hy);
                        //     if hy == ty { println!("Should never happen.");}
                        //     if ((hy.abs() - ty.abs()) - 1).abs() > 0 {
                        //         /* move towards head */
                        //         if ty < hy {
                        //             ty += 1; // ty+(hy-ty-1);
                        //         }else {
                        //             ty -= 1;// ty+(hy-ty+1);
                        //         }
                        //         loggy::debug!("Adjusted Tail on Y: T:{},{}; H: {},{}",tx,ty, hx,hy);
                        //     }
                        // }
                        // else if hy == ty { /* in same col, check row */
                        //     println!("Expecting Row Move.");
                        //     if hx == tx { println!("Should never happen.");}
                        //     if ((hx.abs() - tx.abs()) - 1).abs() > 0 {
                        //         /* move towards head */
                        //         if tx < hx {
                        //             tx += 1; // tx+(hx-tx+1); 
                        //         }else{
                        //             tx -= 1; // tx+(hx-tx-1); 
                        //         }
                        //         loggy::debug!("Adjusted Tail on X: T:{},{}: H: {},{}",tx,ty, hx,hy);
                        //     }
                        // }
                        // else{ /* must be diag */
                        //     /*
                        //         . T . T .
                        //         T . t . T
                        //         . t H t .
                        //         T . t . T
                        //         . T . T .
                        //      */
                        //     println!("Expecting Diag Move.");

                        //     if (hx.abs() - tx.abs()).abs() == 1 && (hy.abs() - ty.abs()).abs() == 1 { /* do nothing */}
                        //     else if hx.abs() - tx.abs() == 1 && hy.abs() - ty.abs() == 2 { 
                        //         tx = hx;
                        //         if hy> ty { ty+=1; }else{ ty -=1; }
                        //         loggy::debug!("Adjusted Tail on Daig: T:{},{} H: {},{}",tx,ty, hx,hy);
                        //         /*
                        //         . T . T .
                        //         . . t . .
                        //         . . H . .
                        //         . . t . .
                        //         . T . T .
                        //         */
                        //     }
                        //     else if (hx.abs() - tx.abs()).abs() == 2 && (hy.abs() - ty.abs()).abs() == 1 { 
                        //         ty = hy;
                        //         if hx > tx { tx+=1; }else{ tx -=1; }
                        //         loggy::debug!("Adjusted Tail on Diag: T:{},{} H: {},{}",tx,ty, hx,hy);
                        //         /*
                        //         . . . . .
                        //         T . . . T
                        //         . t H t .
                        //         T . . . T
                        //         . . . . .
                        //         */
                        //     }else{
                        //         loggy::warn!("[{}] Couldnt MOVE DIAG: T:{},{} H: {},{}","WARN".on_red(),tx,ty, hx,hy);
                        //     }
                        //     // else move diag but needs to be more than 1 off; perf diag is ok.
                            
                        //     // move from T to t location diag
                        //     // if abs(hx)-abs(tx) == 1 and abs(ty)-abs(ty)==1, ok
                        //     // else correct it

                        //     // if tail off by 1 on axis move inward 1
                        // }
                    }
                    // always log the tail location.
                    {
                        let key:String = format!("{}_{}",tx,ty);
                        let mut found = false;
                        for poop in &poopzone {
                            if poop == &*key { found=true; break; }
                        }
                        if !found {
                            loggy::debug!("Poop Added {}", key.to_string().on_red());
                            poopzone.push(key);
                        }
                    }

                    loggy::debug!("=== END LOOP {}: H:{},{} T:{},{}", line_counter, hx,hy, tx,ty);

                    if (hx.abs()-tx.abs()).abs() > 1 {
                        panic!("something wrong1");
                    }
                    if (hy.abs()-ty.abs()).abs() > 1 {
                        panic!("something wrong2");
                    }
                    if action_ctr>0 { /* keep looping */ }else { continue 'nextline; }

                }

                loggy::debug!("=== END STEP {}: H:{},{} T:{},{}", line_counter, hx,hy, tx,ty);
            }
        }
        loggy::debug!("Processed {} lines ", line_counter);   
        outside_counter=line_counter; 
    }
    //println!("{:?}",poopzone);
    let final_output = poopzone.len().to_string();

    return final_output.to_string();

}

#[loggy::scope]
fn star2(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);

    let mut poopzone:Vec<String> = Vec::new();

    let mut hx:i32 = 100000;
    let mut hy:i32 = 100000;
    let mut tx:i32 = 100000;
    let mut ty:i32 = 100000;

    let mut tx2:i32 = 100000;
    let mut ty2:i32 = 100000;
    
    let mut tx3:i32 = 100000;
    let mut ty3:i32 = 100000;
    
    let mut tx4:i32 = 100000;
    let mut ty4:i32 = 100000;
    
    let mut tx5:i32 = 100000;
    let mut ty5:i32 = 100000;
    
    let mut tx6:i32 = 100000;
    let mut ty6:i32 = 100000;
    
    let mut tx7:i32 = 100000;
    let mut ty7:i32 = 100000;
    
    let mut tx8:i32 = 100000;
    let mut ty8:i32 = 100000;
    
    let mut tx9:i32 = 100000;
    let mut ty9:i32 = 100000;
    

    let mut sx:i32 = 0;
    let mut sy:i32 = 0;

    let mut outside_counter = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        
        let mut line_counter: i32 = 0;
        'nextline: for line in lines {
            if let Ok(val) = line {
                line_counter+=1;
                //loggy::debug!("=== BEGIN STEP {}: H:{},{} T:{},{}", line_counter, hx,hy, tx,ty);

                let mut loop_counter:i32 = 4;

                // parse direction and actionctr
                let mut actions:Vec<&str> = val.split(" ").collect();
                //println!("{:?}", actions);
                let action = actions[0];
                let mut action_ctr:i32 = actions[1].parse::<i32>().unwrap();

                loop {
                    loop_counter+=1;action_ctr-=1;

                    // loggy::debug!("Loop {}: Action: {} CTR: {} Line:{}", loop_counter, action, action_ctr, val.green());
                    // loggy::debug!("Pre Move: T:{},{} H: {},{}",tx,ty, hx,hy);

                    match action {
                        "R" => { // move right 1
                            //loggy::debug!("Going Right.");
                            hx += 1;
                        }
                        "L" => {  // move left 1
                            //loggy::debug!("Going Left.");
                            hx -= 1;
                        }
                        "U" => { // move up 1
                            //loggy::debug!("Going Up.");
                            hy += 1;
                        }
                        "D" => { // move down 1
                            //loggy::debug!("Going Down.");
                            hy -= 1;
                        }
                        _ => {panic!("Invalid Action {}", action);}

                    }
                    /*
                    ..T#..
                    ..H##.
                    .####.
                    ....#.
                    s###..
                    H: 1,-3 T:2,-4
                    */
                    // correct the tail location
                    {
                        (tx,ty)   = correctTail(hx, hy, tx, ty);
                        (tx2,ty2) = correctTail(tx, ty, tx2,ty2);
                        (tx3,ty3) = correctTail(tx2,ty2,tx3,ty3);
                        (tx4,ty4) = correctTail(tx3,ty3,tx4,ty4);
                        (tx5,ty5) = correctTail(tx4,ty4,tx5,ty5);
                        (tx6,ty6) = correctTail(tx5,ty5,tx6,ty6);
                        (tx7,ty7) = correctTail(tx6,ty6,tx7,ty7);
                        (tx8,ty8) = correctTail(tx7,ty7,tx8,ty8);
                        (tx9,ty9) = correctTail(tx8,ty8,tx9,ty9);
                        
                    }
                    // always log the tail location.
                    {
                        let key:String = format!("{}_{}",tx9,ty9);
                        let mut found = false;
                        for poop in &poopzone {
                            if poop == &*key { found=true; break; }
                        }
                        if !found {
                            loggy::debug!("Poop Added {}", key.to_string().on_red());
                            poopzone.push(key);
                        }
                    }

                    //loggy::debug!("=== END LOOP {}: H:{},{} T:{},{}", line_counter, hx,hy, tx,ty);

                    if (hx.abs()-tx.abs()).abs() > 1 {
                        panic!("something wrong1");
                    }
                    if (hy.abs()-ty.abs()).abs() > 1 {
                        panic!("something wrong2");
                    }
                    if action_ctr>0 { /* keep looping */ }else { continue 'nextline; }

                }

                //loggy::debug!("=== END STEP {}: H:{},{} T:{},{}", line_counter, hx,hy, tx,ty);
            }
        }
        loggy::debug!("Processed {} lines ", line_counter);   
        outside_counter=line_counter; 
    }
    //println!("{:?}",poopzone);
    let final_output = poopzone.len().to_string();

    return final_output.to_string();
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // log::set_logger(&loggy::Loggy {
    //     prefix: "AoC", // Typically, the name of the program.
    //     show_time: true, // Or false, if you prefer.
    //     show_thread: true, // Or false, if you prefer.
    // }).unwrap();
    // log::set_max_level(log::LevelFilter::Debug); // Or whatever level you want.


    #[test]
    fn test_checktail() {
        // same spot
        assert_eq!(correctTail(0,0,0,0) , (0,0));

        // left and right -no move
        assert_eq!(correctTail(1,0,0,0) , (0,0));
        assert_eq!(correctTail(-1,0,0,0), (0,0));
        assert_eq!(correctTail(0,0,1,0) , (1,0));
        assert_eq!(correctTail(0,0,-1,0), (-1,0));
        // LR - move
        assert_eq!(correctTail(2,0,0,0) , (1,0));
        assert_eq!(correctTail(-2,0,0,0), (-1,0));
        assert_eq!(correctTail(0,0,2,0) , (1,0));
        assert_eq!(correctTail(0,0,-2,0), (-1,0));
        
        // up and down - no move
        assert_eq!(correctTail(0,1,0,0) , (0,0));
        assert_eq!(correctTail(0,-1,0,0), (0,0));
        assert_eq!(correctTail(0,0,0,1) , (0,1));
        assert_eq!(correctTail(0,0,0,-1), (0,-1));
        // UD - move
        assert_eq!(correctTail(0,2,0,0) , (0,1));
        assert_eq!(correctTail(0,-2,0,0), (0,-1));
        assert_eq!(correctTail(0,0,0,2) , (0,1));
        assert_eq!(correctTail(0,0,0,-2), (0,-1));

        //diag - nomove
        assert_eq!(correctTail(1,1,0,0) ,  (0,0));
        assert_eq!(correctTail(1,-1,0,0),  (0,0));
        assert_eq!(correctTail(-1,1,0,0),  (0,0));
        assert_eq!(correctTail(-1,-1,0,0), (0,0));

        assert_eq!(correctTail(0,0,1,1) ,  (1,1));
        assert_eq!(correctTail(0,0,1,-1),  (1,-1));
        assert_eq!(correctTail(0,0,-1,1),  (-1,1));
        assert_eq!(correctTail(0,0,-1,-1), (-1,-1));

        //     /*
        //         . H . H .
        //         H t . t H
        //         . . H . .
        //         H t . t H
        //         . H . H .
        //      */
        //diag - diag move
        assert_eq!(correctTail(1,2,0,0) ,  (1,1));
        assert_eq!(correctTail(1,-2,0,0),  (1,-1));
        assert_eq!(correctTail(-1,2,0,0),  (-1,1));
        assert_eq!(correctTail(-1,-2,0,0), (-1,-1));

        assert_eq!(correctTail(2,1,0,0) ,  (1,1));
        assert_eq!(correctTail(2,-1,0,0),  (1,-1));
        assert_eq!(correctTail(-2,1,0,0),  (-1,1));
        assert_eq!(correctTail(-2,-1,0,0), (-1,-1));

        //     /*
        //         . T . T .
        //         T . t . T
        //         . t H t .
        //         T . t . T
        //         . T . T .
        //      */
        assert_eq!(correctTail(0,0,1,2) ,  (0,1));
        assert_eq!(correctTail(0,0,1,-2),  (0,-1));
        assert_eq!(correctTail(0,0,-1,2),  (0,1));
        assert_eq!(correctTail(0,0,-1,-2), (0,-1));

        assert_eq!(correctTail(0,0,2,1) ,  (1,0));
        assert_eq!(correctTail(0,0,2,-1),  (1,0));
        assert_eq!(correctTail(0,0,-2,1),  (-1,0));
        assert_eq!(correctTail(0,0,-2,-1), (-1,0));

        assert_eq!(correctTail(38,0,38,1), (38,1));
        assert_eq!(correctTail(38,-1,38,1), (38,0));
        assert_eq!(correctTail(38,-2,38,0), (38,-1));

        // H:38,0 T:38,1
        // AoC[0]: 2022-12-12 00:01:36.681 [DEBUG] src/main.rs:240: star1: Going Down.
        // AoC[0]: 2022-12-12 00:01:36.681 [DEBUG] src/main.rs:344: star1: === END LOOP 282: H:38,-1 T:38,1
        // AoC[0]: 2022-12-12 00:01:36.681 [DEBUG] src/main.rs:240: star1: Going Down.
        // AoC[0]: 2022-12-12 00:01:36.681 [DEBUG] src/main.rs:344: star1: === END LOOP 282: H:38,-2 T:38,1
        // AoC[0]: 2022-12-12 00:01:36.681 [DEBUG] src/main.rs:240: star1: Going Down.
        // y off by 2; adjusting.
        // AoC[0]: 2022-12-12 00:01:36.681 [DEBUG] src/main.rs:339: star1: Poop Added 38_0
        // AoC[0]: 2022-12-12 00:01:36.681 [DEBUG] src/main.rs:344: star1: === END LOOP 282: H:38,-3 T:38,0

    }

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(correctTail(1, 2), 3);
    // }
}