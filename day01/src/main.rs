fn main() {
    let input = include_str!("input.txt");
    let changes: Vec<isize> = input
        .trim()
        .split("\n")
        .map(|x| parse(x))
        .collect::<Vec<_>>();
    println!("Part 1: {}", changes.iter().sum::<isize>());
}

fn parse(line: &str) -> isize {
    line.parse().unwrap()
}

#[test]
fn test_parse() {
    assert_eq!(parse("+10"), 10isize);
    assert_eq!(parse("-10"), -10isize);
}
