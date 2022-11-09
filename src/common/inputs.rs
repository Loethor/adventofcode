use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn read_numbers(year: u32, day: u8) -> Vec<i64> {
    let path = format!("./inputs/{}/day{:02}.txt", year, day);
    let file = File::open(path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    return numbers;
}

pub fn read(year: u32, day: u8) -> Vec<String> {
    let mut r = Vec::with_capacity(1000);
    let path = format!("./inputs/{}/day{:02}.txt", year, day);
    let fp = File::open(&path).expect(&format!("Can't open {}", path));
    let fp = BufReader::new(fp);
    for line in fp.lines() {
        r.push(line.unwrap());
    }
    return r;
}