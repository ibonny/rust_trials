use std::fs;

fn count_lines(input: &Vec<&str>) -> usize {
    let mut num: usize = 0;

    for _ in input {
        num += 1
    }

    return num;
}

#[tokio::main]
async fn main() {
    let file_path = "src/poem.txt".to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to open the file.");

    let lines: Vec<&str> = contents.split('\n').collect();

    let lines2 = &lines;

    println!("{:?}", lines);

    println!("{:?}", count_lines(lines2));

    let res = reqwest::get("https://jsonplaceholder.typicode.com/users")
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    if let Some(arr) = res.as_array() {
        for obj in arr {
            println!("{:?}", obj["name"].as_str().expect("Not available."));
        }
    }
}
