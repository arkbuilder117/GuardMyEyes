// use std::env;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
use std::process::exit;
use std::time::Instant;

use console::Term;

fn main() -> ! {
    // let file_path = Path::new("/etc/hosts");
    // let display = file_path.display();

    // let contents = fs::read_to_string(file_path).expect("Should read file");

    // println!("With text:\n{contents}");

    // let mut host_file = match File::open(file_path) {
    //     Err(why) => panic!("Couldn't open {display}: {why}"),
    //     Ok(file) => file,
    // };

    // let mut content = String::new();
    // match host_file.read_to_string(&mut content) {
    //     Err(why) => panic!("Couldn't read {display}: {why}"),
    //     Ok(_) => print!("{display} contains:\n{content}"),
    // };

    let now = Instant::now();
    let mut blocklist: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./src/blocklist.txt") {
        for line in lines {
            if let Ok(website) = line {
                print!("{website}");
                blocklist.push(website);
            }
        }
    }

    print!("Contents: {blocklist:?}\n");

    loop {
        let cur_time = now.elapsed();
        print!("Timer: {}\n", cur_time.as_secs());
        print!("Blocklist:\n");
        display_blocklist(&blocklist);
        print!(
            "Actions: \nB: Add or delete from blocklist\nE: Extend Timer\nQ: quit\nEnter choice:\n"
        );
        let term = Term::stdout();

        match Term::read_char(&term) {
            Ok('B') | Ok('b') => println!("B\n"),
            Ok('E') | Ok('e') => println!("E\n"),
            Ok('Q') | Ok('q') => exit(0),
            Ok(_) => println!("Not an option. Enter again!\n"),
            Err(_) => todo!(),
        }
    }
}

fn display_blocklist(list: &[String]) {
    for i in list {
        print!("{i}\n");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    // let file = match File::open(filename) {
    //     Ok(file) => io::BufReader::new(file).lines(),
    //     Err(why) => panic!("Couldn't open file"),
    // };
    // Ok(io::BufReader::new(file).lines())

    let file = File::open(filename).expect("file did not open");
    Ok(io::BufReader::new(file).lines())
}
