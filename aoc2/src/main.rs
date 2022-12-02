use std::char;
use std::{fs::File, io::{BufRead, BufReader}};

fn process_lines(reader: BufReader<File>) {
    for line in reader.lines() {
        let line = line.unwrap(); // unwrap the Result to get the line

        // compare the first and last characters of the line
        let first_char = line.chars().next().unwrap();
        let last_char = line.chars().rev().next().unwrap();
        let mut total = 0;
        if first_char == A {
           match last_char {
             X => total = total + 
             Y =>
             Z =Z
           }
        } else {
            println!("The first and last characters are not equal");
        }

        // perform arithmetic on the first and last characters
        let n1 = first_char; // convert to u32
        let n2 = last_char; // convert to u32

        println!("Outputs: {} {})",n1,n2);
    }
}

fn main() {
    let file = File::open("./input/2.txt").unwrap();
    let reader = BufReader::new(file);

    process_lines(reader);
}
