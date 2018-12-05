// --- Part Two ---

// Strategy 2: Of all guards, which guard is most frequently asleep on the same
// minute?

// In the example above, Guard #99 spent minute 45 asleep more than any other
// guard or minute - three times in total. (In all other cases, any guard spent
// any minute asleep at most twice.)

// What is the ID of the guard you chose multiplied by the minute you chose? (In
// the above example, the answer would be 99 * 45 = 4455.)

use std::io;
use std::str::FromStr;
use std::collections::HashMap;

enum Entry {
	BeginsShift(usize),
	FallsAsleep(usize),
	WakesUp(usize),
}
use Entry::*;

impl FromStr for Entry {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let minute_start = "[1518-04-11 00:".len();
		let minute = usize::from_str(&s[minute_start .. minute_start + 2]).unwrap();

		let message = &s["[1518-04-11 00:44] ".len() .. ];
		Ok(if message == "wakes up" {
			WakesUp(minute)
		} else if message == "falls asleep" {
			FallsAsleep(minute)
		} else {
			let id_end = message.find(" begins shift").unwrap();
			BeginsShift(usize::from_str(&message["Guard #".len() .. id_end]).unwrap())
		})
	}
}

fn select_guard_and_minute(entries: impl Iterator<Item=Entry>) -> (usize, usize) {
	let sleep_data = guards_sleep_data(entries);

	let (&guard_id, (minute, _)) = sleep_data.iter()
		.map(|(guard_id, times_asleep_in_minute)| (
			guard_id, 
			times_asleep_in_minute.iter()
				.enumerate()
				.max_by_key(|(_minute, freq)| freq.clone())
				.unwrap(),
		))
		.max_by_key(|(_guard_id, (_minute, freq))| freq.clone())
		.unwrap();

	(guard_id, minute)
}

type TimesAsleepInMinute = [usize; 60];

fn guards_sleep_data(entries: impl Iterator<Item=Entry>) -> HashMap<usize, TimesAsleepInMinute> {
	let mut times_asleep_in_minute = HashMap::new();

	let mut current = None;
	let mut fell_at = None;

	for entry in entries {
		match entry {
			BeginsShift(guard) => { current = Some(guard) },
			FallsAsleep(m) => { fell_at = Some(m) },
			WakesUp(waked_up) => {
				let fell_at = fell_at.unwrap();

				let guard = match &current {
					&Some(ref s) => s.clone(),
					&None => unreachable!(),
				};

				let guard_times = times_asleep_in_minute.entry(guard).or_insert([0; 60]);

				for minute_asleep in fell_at .. waked_up {
					guard_times[minute_asleep] += 1;
				}
			},
		}
	}

	times_asleep_in_minute
}

fn main() {
	use io::BufRead;
	let stdin = io::stdin();
	let (guard, minute) = select_guard_and_minute(
		stdin.lock()
			.lines()
			.map(|s| s.unwrap())
			.sort()
			.map(|v| Entry::from_str(&v))
			.map(|e| e.ok().unwrap())
	);
	println!("{}", guard * minute);
}

trait Sort: Iterator {
	fn sort(self) -> std::vec::IntoIter<Self::Item>
		where
			Self::Item: Ord,
			Self: Sized,
	{
		let mut v: Vec<Self::Item> = self.collect();
		v.sort();
		v.into_iter()
	}
}

impl <T> Sort for T where T: Iterator {}
