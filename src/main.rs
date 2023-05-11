use std::{io::{BufReader, BufRead}, fs::File};


fn main() {

    let reader = BufReader::new(File::open("input.txt").unwrap());

    let (_, count) = reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap()).fold(
        (vec![], 0), |(mut state, count), _i| {

            state.push(_i);

            if state.len() < 4 {
                return (state, count);
            } 
            if state.len() == 5 {
                state.remove(0);
            }

            if state[0] < state[3] {
                return (state, count + 1);
            } else {
                return (state, count);
            }
        }
    );

    println!("Number of increases {}", count);
}
