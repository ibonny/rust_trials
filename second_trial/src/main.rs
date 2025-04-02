use std::fs;

fn count_lines(input: &Vec<&str>) -> usize {
    let mut num: usize = 0;

    for _ in input {
        num += 1
    }

    return num;
}

fn main() {
    let file_path = "src/poem.txt".to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to open the file.");

    let lines: Vec<&str> = contents.split('\n').collect();

    let lines2 = &lines;

    println!("{:?}", lines);

    println!("{:?}", count_lines(lines2));

    let res = reqwest::blocking::get("https://jsonplaceholder.typicode.com/users")
        .expect("blah")
        .text();

    println!("{:?}", res);
}
