use std::{convert::From, env, fmt};
use chrono::prelude::*;

const SECS_PER_MINUTE: u32 = 60;
const MINUTES_PER_HOUR: u32 = 60;
const SECS_PER_HOUR: u32 = SECS_PER_MINUTE * MINUTES_PER_HOUR;
const DECI_SECS_PER_MINUTE: u32 = 100;
const DECI_MINUTES_PER_HOUR: u32 = 40;
const DECI_SECS_PER_HOUR: u32 = DECI_MINUTES_PER_HOUR * DECI_SECS_PER_MINUTE;

const MAN_PAGE: &str =
  "NAME\n\
  \tdecitime - convert local time to decimal time\n\
  \n\
  SYNOPSIS\n\
  \tdecitime [hh:mm:ss]\n\
  \n\
  DESCRPTION\n\
  \tConvert local time to decimal time, \
  assuming each day is 25 hours * 40 minutes * 100 seconds\n";


struct DeciTime {
  hour: u32,
  minute: u32,
  second: u32
}


impl DeciTime {
  pub fn from_hms(hh: u32, mm: u32, ss: u32) -> Self {
  	let day_secs = hh * SECS_PER_HOUR
  		+ mm * SECS_PER_MINUTE
  		+ ss;
  	// 86400 day secs = 100000 deci day secs
  	let decimal_secs = day_secs * 500 / 432;
  	let hour = decimal_secs / DECI_SECS_PER_HOUR;
  	let hour_secs = decimal_secs % DECI_SECS_PER_HOUR;
  	let minute = hour_secs / DECI_SECS_PER_MINUTE;
  	let second = hour_secs % DECI_SECS_PER_MINUTE;
  	Self{hour, minute, second}
  }
}


impl From<DateTime<Local>> for DeciTime {
  fn from(time: DateTime<Local>) -> Self {
    DeciTime::from_hms(time.hour(), time.minute(), time.second())
  }
}


impl From<NaiveTime> for DeciTime {
  fn from(time: NaiveTime) -> Self {
    DeciTime::from_hms(time.hour(), time.minute(), time.second())
  }
}


impl fmt::Display for DeciTime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
  	write!(f, "{:02}:{:02}:{:02}",
  		self.hour, self.minute, self.second)
  }
}


fn main() {
  let args_count = env::args().count();
  if args_count == 1 {
  	println!("{}", DeciTime::from(Local::now()));
  } else if args_count == 2 {
  	let arg = env::args()
  		.nth(1)
  		.expect("Provide local time");
  	let naive_time = NaiveTime::parse_from_str(&*arg, "%H:%M:%S")
  		.expect("Provide local time as HH:MM:SS");
  	println!("{}", DeciTime::from(naive_time));	  
  } else {
  	eprintln!("{}", MAN_PAGE);
  }
}
