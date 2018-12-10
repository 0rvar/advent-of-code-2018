use shared::*;

fn main() {
    // let line_regex = Regex::new(
    //     r"\[1518-(\d+)-(\d+) (\d+):(\d+)\] (Guard #\d+ begins shift|falls asleep|wakes up)",
    // )
    // .unwrap();
    // let guard_regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    // for line in lines {
    //     let captures = line_regex.captures(line).unwrap();
    //     let month = get_number_match(&captures, 1);
    let input: Vec<(char, char)> = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|x| {
            let parts = x
                .split(" ")
                .filter(|x| x.len() == 1)
                .map(|x| x.chars().next().unwrap())
                .collect::<Vec<char>>();

            (parts[0], parts[1])
        })
        .collect();
}
