use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let result = line
        .trim()
        .split(' ')
        .flat_map(&str::parse::<i32>)
        .fold(0, |acc, curr| acc + curr);

    println!("{}", result);
}
