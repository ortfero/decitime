use chrono::prelude::*;
use ctrlc;
use std::{env, io::{self, Write}, thread, time::Duration, process};

mod decitime;
use decitime::DeciTime;


const MAN_PAGE: &str =
  "NAME\n\
  \tdecitime - convert local time to decimal time\n\
  \n\
  SYNOPSIS\n\
  \tdecitime [OPTIONhh:mm:ss]\n\
  \n\
  DESCRPTION\n\
  \tConvert curent local time to decimal time, \
  assuming each day is 100 hours * 100 minutes * 10 seconds\n\
  \t-c, --continue\n\
  \t\tprint decimal time continuously (press Ctrl+C to quit)\n\
  \thh:mm:ss\n
  \t\tconvert specified day time to decimal time\n";

fn print_time_continuosly() {
  ctrlc::set_handler(|| {
    process::exit(0);
  })
  .expect("Error setting Ctrl-C handler");  
  loop {
    print!("{}", DeciTime::from(Local::now()));
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(1));
    print!("\r");
  }
}


fn convert_local_time(hms: &str) {
  let naive_time = NaiveTime::parse_from_str(hms, "%H:%M:%S")
    .expect("Provide local time as HH:MM:SS");
  println!("{}", DeciTime::from(naive_time));
}


fn handle_argument() {
  let arg = env::args()
    .nth(1)
    .expect("Provide option");
  match arg.as_str() {
    "-c" | "--continue" => print_time_continuosly(),
    _ => convert_local_time(arg.as_str())
  };
}

fn main() {
  match env::args().count() {
    1 => println!("{}", DeciTime::from(Local::now())),
    2 => handle_argument(),
    _ => eprintln!("{}", MAN_PAGE)
  }
}
