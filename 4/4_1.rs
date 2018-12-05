// --- Day 4: Repose Record ---

// You've sneaked into another supply closet - this time, it's across from the
// prototype suit manufacturing lab. You need to sneak inside and fix the issues
// with the suit, but there's a guard stationed outside the lab, so this is as
// close as you can safely get.

// As you search the closet for anything that might help, you discover that
// you're not the first person to want to sneak in. Covering the walls, someone
// has spent an hour starting every midnight for the past few months secretly
// observing this guard post! They've been writing down the ID of the one guard
// on duty that night - the Elves seem to have decided that one guard was enough
// for the overnight shift - as well as when they fall asleep or wake up while
// at their post (your puzzle input).

// For example, consider the following records, which have already been
// organized into chronological order:

// [1518-11-01 00:00] Guard #10 begins shift
// [1518-11-01 00:05] falls asleep
// [1518-11-01 00:25] wakes up
// [1518-11-01 00:30] falls asleep
// [1518-11-01 00:55] wakes up
// [1518-11-01 23:58] Guard #99 begins shift
// [1518-11-02 00:40] falls asleep
// [1518-11-02 00:50] wakes up
// [1518-11-03 00:05] Guard #10 begins shift
// [1518-11-03 00:24] falls asleep
// [1518-11-03 00:29] wakes up
// [1518-11-04 00:02] Guard #99 begins shift
// [1518-11-04 00:36] falls asleep
// [1518-11-04 00:46] wakes up
// [1518-11-05 00:03] Guard #99 begins shift
// [1518-11-05 00:45] falls asleep
// [1518-11-05 00:55] wakes up

// Timestamps are written using year-month-day hour:minute format. The guard
// falling asleep or waking up is always the one whose shift most recently
// started. Because all asleep/awake times are during the midnight hour (00:00 -
// 00:59), only the minute portion (00 - 59) is relevant for those events.

// Visually, these records show that the guards are asleep at these times:

// Date   ID   Minute
//			 000000000011111111112222222222333333333344444444445555555555
//			 012345678901234567890123456789012345678901234567890123456789
// 11-01  #10  .....####################.....#########################.....
// 11-02  #99  ........................................##########..........
// 11-03  #10  ........................#####...............................
// 11-04  #99  ....................................##########..............
// 11-05  #99  .............................................##########.....

// The columns are Date, which shows the month-day portion of the relevant day;
// ID, which shows the guard on duty that day; and Minute, which shows the
// minutes during which the guard was asleep within the midnight hour. (The
// Minute column's header shows the minute's ten's digit in the first row and
// the one's digit in the second row.) Awake is shown as ., and asleep is shown
// as #.

// Note that guards count as asleep on the minute they fall asleep, and they
// count as awake on the minute they wake up. For example, because Guard #10
// wakes up at 00:25 on 1518-11-01, minute 25 is marked as awake.

// If you can figure out the guard most likely to be asleep at a specific time,
// you might be able to trick that guard into working tonight so you can have
// the best chance of sneaking in. You have two strategies for choosing the best
// guard/minute combination.

// Strategy 1: Find the guard that has the most minutes asleep. What minute does
// that guard spend asleep the most?

// In the example above, Guard #10 spent the most minutes asleep, a total of 50
// minutes (20+25+5), while Guard #99 only slept for a total of 30 minutes
// (10+10+10). Guard #10 was asleep most during minute 24 (on two days, whereas
// any other minute the guard was asleep was only seen on one day).

// While this example listed the entries in chronological order, your entries
// are in the order you found them. You'll need to organize them before they can
// be analyzed.

// What is the ID of the guard you chose multiplied by the minute you chose? (In
// the above example, the answer would be 10 * 24 = 240.)

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

	let (&guard_id, GuardSleep { times_asleep_in_minute, .. }) = sleep_data.iter()
		.max_by_key(|(_, GuardSleep { minutes_asleep: m, .. })| m)
		.unwrap();

	let (minute, _) = times_asleep_in_minute.iter().
		enumerate().
		max_by_key(|(_, v)| v.clone()).
		unwrap();

	(guard_id, minute)
}

struct GuardSleep {
	minutes_asleep: usize,
	times_asleep_in_minute: [usize; 60],
}

fn guards_sleep_data(entries: impl Iterator<Item=Entry>) -> HashMap<usize, GuardSleep> {
	let mut sleep_data = HashMap::new();

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

				let guard_data = sleep_data.entry(guard).or_insert(GuardSleep {
					minutes_asleep: 0,
					times_asleep_in_minute: [0; 60],
				});

				guard_data.minutes_asleep += waked_up - fell_at;

				for minute_asleep in fell_at .. waked_up {
					guard_data.times_asleep_in_minute[minute_asleep] += 1;
				}
			},
		}
	}

	sleep_data
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
