use std::str::FromStr;
use chrono::{NaiveDateTime, Timelike};
use chrono::ParseError;
use std::collections::{BTreeMap, HashMap};
use crate::helper;

struct GuardSleepTime {
    most_slept_min: u32,
    most_slept_quantity: u32,
    total_slept_time: u32
}

enum EventType {
    StartedGuard(i32),
    FallAsleep(usize),
    WokeUp(usize)
}

struct LogEntry {
    date_time: NaiveDateTime,
    event: EventType
}

pub fn solve_problem_4(path: &str) -> (i32, i32) {
    let content = helper::get_file_content_as_str(path);
    if content.is_none() { return (-1, -1); }

    let content = content.unwrap();
    let content = content.as_str();
    let ordered_log = order_log(content);
    if ordered_log.is_none() { return (-1, -1); }

    let guards_sleep = process_log(ordered_log.unwrap());
    let result_1 = find_strategy_1(&guards_sleep);
    let result_2 = find_strategy_2(&guards_sleep);
    (result_1, result_2)
}

fn find_strategy_2(guard_sleep: &HashMap<i32, GuardSleepTime>) -> i32 {
    let max_guard = guard_sleep.iter().max_by(|(_, x), (_, y)| {
        x.most_slept_quantity.cmp(&y.most_slept_quantity)
    });

    if max_guard.is_none() { return -1; }
    let (id, sleep_time) = max_guard.unwrap();
    *id * sleep_time.most_slept_min as i32
}

fn find_strategy_1(guards_sleep: &HashMap<i32, GuardSleepTime>) -> i32 {
    let max_guard = guards_sleep.iter().max_by(|(_, x), (_, y)| {
        x.total_slept_time.cmp(&y.total_slept_time)
    });
    if max_guard.is_none() { return -1; }

    let (id, sleep_time) = max_guard.unwrap();
    *id * sleep_time.most_slept_min as i32
}

fn order_log(data: &str) -> Option<BTreeMap<NaiveDateTime, EventType>> {
    let mut map = BTreeMap::new();
    for line in data.lines() {
        let entry = line.parse::<LogEntry>();
        if entry.is_err() { return None; }

        let entry = entry.unwrap();
        map.insert(entry.date_time, entry.event);
    }
    Some(map)
}

fn process_log(data: BTreeMap<NaiveDateTime, EventType>) -> HashMap<i32, GuardSleepTime> {
    let mut map = HashMap::new();
    let mut start_sleeping= 0;
    let mut guard_id = 0;
    for event in data.values() {
        match event {
            EventType::StartedGuard(id) => {
                guard_id = *id;
                map.entry(guard_id).or_insert_with(|| vec![0; 60]);
            },
            EventType::FallAsleep(minute) => start_sleeping = *minute,
            EventType::WokeUp(minute) => {
                let ended_sleeping = minute;
                map.entry(guard_id).and_modify(|histogram| {
                   mark_histogram(histogram, start_sleeping, *ended_sleeping)
                });
            }
        }
    }

    let mut result = HashMap::new();
    let tuples = map.into_iter().map(|(id, histogram)| {
        let most_slept = most_slept(&histogram);
        let sum: u32 = histogram.into_iter().sum();
        let guard = GuardSleepTime {
            total_slept_time: sum,
            most_slept_min: most_slept.0 as u32,
            most_slept_quantity: most_slept.1
        };
        (id, guard)
    });

    for (id, guard) in tuples {
        result.insert(id, guard);
    }
    result
}

fn most_slept(histogram: &[u32]) -> (usize, u32) {
    let mut index = 0usize;
    let mut max = histogram[0];
    for i in 1..histogram.len() {
        if histogram[i] > max {
            index = i;
            max = histogram[i];
        }
    }
    (index, max)
}

fn mark_histogram(histogram: &mut Vec<u32>, start: usize, end: usize) {
    for i in start..end {
        histogram[i] += 1;
    }

}

impl FromStr for LogEntry {
    type Err = ParseError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let time = &line[1..17];

        let date_time = NaiveDateTime::parse_from_str(time,"%Y-%m-%d %H:%M")?;
        let event = String::from(&line[19..]);
        let event = parse_event(event.as_str(), date_time.minute())
            .unwrap_or(EventType::StartedGuard(-1));
        Ok(LogEntry {
            date_time,
            event
        })
    }
}

fn parse_event(event: &str, minute: u32) -> Option<EventType> {
    let first = event.chars().next().unwrap();
    match first {
        'G' => Some(EventType::StartedGuard(get_guard_id(event))),
        'f' => Some(EventType::FallAsleep(minute as usize)),
        'w' => Some(EventType::WokeUp(minute as usize)),
        _ => None
    }
}

fn get_guard_id(event: &str) -> i32 {
    let hash = event.find('#');
    if hash.is_none() { return -1; }

    let substring = &event[hash.unwrap()..];
    let next_space = substring.find(' ');
    if next_space.is_none() { return -1; }

    let substring = &substring[1..next_space.unwrap()];
    substring.parse::<i32>().unwrap()
}