extern crate utils;
extern crate loggy;
use owo_colors::OwoColorize;
use std::collections::HashMap;

use itertools::Itertools;
#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}
#[derive(Debug)]
struct Directory {
    name: String,
    files: HashMap<String, File>,
    directories: HashMap<String, Directory>,
}

impl Directory {

    fn get_files_size (&self) -> u32 {
        self.files.values().map(|file| file.size).sum()
    }

    fn get_total_size (&self) -> u32 {

        let mut size = self.get_files_size();

        for directory in get_all_directories_in(self) {
            size += directory.get_files_size()
        }

        size
    }

}
#[derive(Debug)]
struct FileSystem {
    directories: HashMap<String, Directory>,
}

enum TerminalLine {
    List,
    ChangeDirectory(String),
    File(File),
    Directory(Directory),
}


fn get_current_directory <'a> (file_system: &'a mut FileSystem, pwd: &Vec<String>) -> &'a mut Directory {

    let mut curdir = file_system.directories.get_mut(pwd.first().unwrap()).unwrap();

    for directory in pwd.iter().skip(1) {
        curdir = curdir.directories.get_mut(directory).unwrap()
    }

    curdir
}

fn get_all_directories_in (directory: &Directory) -> Vec<&Directory> {

    let mut directories: Vec<&Directory> = vec!();

    for directory in directory.directories.values() {
        directories.push(directory);
        directories.append(&mut get_all_directories_in(directory))
    }

    directories
}

fn parse_terminal_line (line: &str) -> TerminalLine {

    match &line[0..4] {
        "$ cd" => TerminalLine::ChangeDirectory(line[5..].to_string()),
        "$ ls" => TerminalLine::List,
        "dir " => TerminalLine::Directory(Directory{ name: line[4..].to_string(), files: HashMap::new(), directories: HashMap::new()}),
        _ => {
                let (size, name) = line.split(" ").tuples().next().unwrap();
                let file = File { size: size.parse().unwrap(), name: name.to_string() };
                TerminalLine::File(file)
        }
    }
}

fn part1 (file_system: &FileSystem) -> u32 {

    let mut all_directories = vec!(&file_system.directories["/"]);
    all_directories.append(&mut get_all_directories_in(&all_directories[0]));

    all_directories
        .iter()
        .map(|directory| directory.get_total_size())
        .filter(|size| *size <= 100000 as u32)
        .sum()

}

fn part2 (file_system: &FileSystem) -> u32 {

    let mut all_directories = vec!(&file_system.directories["/"]);
    all_directories.append(&mut get_all_directories_in(&all_directories[0]));

    // The total disk space available to the filesystem is 70000000. 
    // To run the update, you need unused space of at least 30000000. 
    // You need to find a directory you can delete that will free up 
    // enough space to run the update.

    // order dir sizes smallest to largest, then find first greater than 30000000

    // how much free space is there
    let free_space = 70000000 - &file_system.directories["/"].get_total_size();
    
    all_directories.iter()
        .map(|directory| directory.get_total_size())
        .filter(|size| *size + free_space >= 30000000)
        .min()
        .unwrap()

}

fn main() {
    log::set_logger(&loggy::Loggy {
        prefix: "AoC", // Typically, the name of the program.
        show_time: true, // Or false, if you prefer.
        show_thread: true, // Or false, if you prefer.
    }).unwrap();
    log::set_max_level(log::LevelFilter::Debug); // Or whatever level you want.

    loggy::info!("starting up");

    let mut expected_result = "95437";
    utils::evaluate_results(star1("./input01-test.txt".to_string()), expected_result.to_string());
    utils::evaluate_results(star1("./input01-1.txt".to_string()), "-1".to_string()); // 1315285
    
    expected_result = "24933642";
    utils::evaluate_results(star2("./input01-test.txt".to_string()), expected_result.to_string());
    utils::evaluate_results(star2("./input01-1.txt".to_string()), "-1".to_string()); // 12881

    //let file_system: FileSystem = generate_filesystem();
    //println!("The answer to Part 1 is {}", part1(&file_system));
    //println!("The answer to Part 2 is {}", part2(&file_system));

}


#[loggy::scope]
fn star1(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);
    //let file_system: FileSystem = generate_filesystem();

    /////////////////
    //let lines: Vec<&str> = include_str!("data/data.txt").lines().collect();
    let mut file_system: FileSystem = FileSystem { directories: HashMap::new() };

    let mut pwd: Vec<String> = vec!(String::from("/"));

    let root: Directory = Directory{ name: String::from("/"), files: HashMap::new(), directories: HashMap::new()};
    file_system.directories.insert(root.name.to_string(), root);

    if let Ok(lines) = utils::read_lines(file_path) {
        let mut line_counter: i32 = 0;
        for line in lines {
            if let Ok(val) = line {
             
                //println!("{}", val.green());

                match parse_terminal_line(&*val) {
                    TerminalLine::List => {},
                    TerminalLine::ChangeDirectory(name) => {
                        if name == "/" {
                            continue;
                        }else if name == ".." {
                            pwd = pwd[0..pwd.len() - 1].to_vec()
                        } else {
                            pwd.push(name)
                        }
                    }
                    TerminalLine::File(file) => {
                        get_current_directory(&mut file_system, &pwd).files.insert(file.name.to_string(), file);
                    }
                    TerminalLine::Directory(directory) => {
                        get_current_directory(&mut file_system, &pwd).directories.insert(directory.name.to_string(), directory);
                    }
                }
                line_counter+=1;
            }
        }
        loggy::debug!("Processed {} lines ", line_counter); 

    }

    return part1(&file_system).to_string();
}

#[loggy::scope]
fn star2(file_path: String) -> String {
    loggy::info!("Loading File: {}", file_path);
    //let file_system: FileSystem = generate_filesystem();

    /////////////////
    //let lines: Vec<&str> = include_str!("data/data.txt").lines().collect();
    let mut file_system: FileSystem = FileSystem { directories: HashMap::new() };

    let mut pwd: Vec<String> = vec!(String::from("/"));

    let root: Directory = Directory{ name: String::from("/"), files: HashMap::new(), directories: HashMap::new()};
    file_system.directories.insert(root.name.to_string(), root);

    if let Ok(lines) = utils::read_lines(file_path) {
        let mut line_counter: i32 = 0;
        for line in lines {
            if let Ok(val) = line {
             
                //println!("{}", val.green());

                match parse_terminal_line(&*val) {
                    TerminalLine::List => {},
                    TerminalLine::ChangeDirectory(name) => {
                        if name == "/" {
                            continue;
                        }else if name == ".." {
                            pwd = pwd[0..pwd.len() - 1].to_vec()
                        } else {
                            pwd.push(name)
                        }
                    }
                    TerminalLine::File(file) => {
                        get_current_directory(&mut file_system, &pwd).files.insert(file.name.to_string(), file);
                    }
                    TerminalLine::Directory(directory) => {
                        get_current_directory(&mut file_system, &pwd).directories.insert(directory.name.to_string(), directory);
                    }
                }
                line_counter+=1;
            }
        }
        loggy::debug!("Processed {} lines ", line_counter); 

    }

    return part2(&file_system).to_string();
}
