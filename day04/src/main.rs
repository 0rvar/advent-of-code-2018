use itertools::Itertools;
use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum LogEvent {
    WakeUp,
    FallAsleep,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct LogEntry {
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
    guard_id: usize,
    event: LogEvent,
}

fn main() {
    let lines = include_str!("input.txt").trim().lines().collect::<Vec<_>>();

    let (logs, guard_ids) = {
        let mut guard_id = 0usize;
        let mut guard_ids = HashSet::new();
        let mut logs: Vec<LogEntry> = vec![];
        let line_regex = Regex::new(
            r"\[1518-(\d+)-(\d+) (\d+):(\d+)\] (Guard #\d+ begins shift|falls asleep|wakes up)",
        )
        .unwrap();
        let guard_regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();

        for line in lines {
            let captures = line_regex.captures(line).unwrap();
            let month = get_number_match(&captures, 1);
            let day = get_number_match(&captures, 2);
            let hour = get_number_match(&captures, 3);
            let minute = get_number_match(&captures, 4);
            let event = match get_string_match(&captures, 5) {
                "falls asleep" => LogEvent::FallAsleep,
                "wakes up" => LogEvent::WakeUp,
                x => {
                    let guard_captures = guard_regex.captures(&x).unwrap();
                    guard_id = get_number_match(&guard_captures, 1);
                    LogEvent::WakeUp
                }
            };
            logs.push(LogEntry {
                month,
                day,
                hour,
                minute,
                event,
                guard_id,
            });
            guard_ids.insert(guard_id);
        }
        (logs, guard_ids)
    };

    let mut grouped = HashMap::new();
    for guard_id in guard_ids {
        let mut guard_logs = vec![];
        for log in &logs {
            if log.guard_id == guard_id {
                guard_logs.push(log);
            }
        }
        grouped.insert(guard_id, guard_logs);
    }

    let mut max_asleep_minutes = 0usize;
    let mut max_asleep_minutes_guard_id = 0usize;
    let mut max_asleep_minutes_time = 0usize;
    for (guard_id, guard_events) in grouped {
        let mut go_to_sleep_minute = 0usize;
        let mut asleep_minutes: HashMap<usize, usize> = HashMap::new();
        for log in guard_events {
            match log.event {
                LogEvent::WakeUp => {
                    let wake_minute = if log.hour != 0 { 0 } else { log.minute };
                    for minute in go_to_sleep_minute..wake_minute {
                        let asleep_entry = asleep_minutes.entry(minute).or_insert(0usize);
                        *asleep_entry += 1;
                    }
                }
                LogEvent::FallAsleep => {
                    go_to_sleep_minute = log.minute;
                }
            }
        }
        let total_minutes_asleep: usize = asleep_minutes.values().sum();
        let guard_popular_minute = if asleep_minutes.len() > 0 {
            let mut max = 0usize;
            let mut max_minute = 0usize;
            for (minute, x) in asleep_minutes {
                if x > max {
                    max = x;
                    max_minute = minute;
                }
            }
            if guard_id == 2287 {
                println!("{}, {}", max, max_minute);
            }
            max_minute
        } else {
            0
        };
        if total_minutes_asleep > max_asleep_minutes {
            max_asleep_minutes = total_minutes_asleep;
            max_asleep_minutes_time = guard_popular_minute;
            max_asleep_minutes_guard_id = guard_id;
        }
    }

    println!(
        "minutes: {}, most popular minute: {}, guard_id: {}, (*: {})",
        max_asleep_minutes,
        max_asleep_minutes_time,
        max_asleep_minutes_guard_id,
        max_asleep_minutes_guard_id * max_asleep_minutes_time
    );
}

fn get_number_match(capture: &Captures<'_>, index: usize) -> usize {
    capture.get(index).unwrap().as_str().parse().unwrap()
}

fn get_string_match<'l>(capture: &Captures<'l>, index: usize) -> &'l str {
    capture.get(index).unwrap().as_str()
}
