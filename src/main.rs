use std::env;
use chrono::prelude::*;

mod decitime;
use decitime::DeciTime;


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
