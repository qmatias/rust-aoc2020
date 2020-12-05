use std::io::BufReader;
use std::io::BufRead;

use crate::AoCDay;

pub struct Code;

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let reader = BufReader::new(_input);
        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
        todo!()
    }

    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        todo!()
    }
}
