// the crate for my own time format
mod time;

// used to get the local time
use chrono::Local;

// used to get the input from the command line when running the program, will be deprecated once
// the gui has been done
use std::env::args;

use time::timing::Time;

fn main() {
    let local_time = Local::now().format("%H:%M").to_string();
    let local_time: Time = Time::from_string(local_time);

    let argument = args()
        .nth(1)
        .expect("read the argument provided at compile time");
    let desired_time = Time::from_string(argument);

    println!("\n\n");
    println!("+-----------------------------------------------------------------+");
    println!("| if you want to wake up at {:<38}|", desired_time);
    println!("+-----------------------------------------------------------------+");
    for sleep_cycle in 1..10 {
        // from my research 1 sleep cycle is 90 minutes long, that means I will subtract n * 90
        // minutes for the nth sleep cycle
        let time_to_subtract = Time {
            hour: 1,
            minute: 30,
        } * (sleep_cycle as u8);

        let time_to_fall_asleep = desired_time - time_to_subtract;

        // here I subtract the time it takes a person on average to fall asleep (once the gui is
        // done this will probably be a toggle or I will display both values)
        let time_to_fall_asleep = time_to_fall_asleep - Time::from_string("00:14".to_string());
        let time_till_fall_asleep = time_to_fall_asleep - local_time;

        // we ignore any bedtime that is more than 12 hours from now because that'd just be the
        // bedtime for the next day then (though now writing this if someone is planning ahead
        // their sleep this just kinda works against them)
        if time_till_fall_asleep.hour > 12 {
            continue;
        }

        println!(
            "| for {} sleep cycles you have to to bed at {} (in {}){:<7}|",
            sleep_cycle, time_to_fall_asleep, time_till_fall_asleep, ""
        );
    }

    println!("+-----------------------------------------------------------------+");
    println!("| current time is: {:<47}|", local_time);
    println!("+-----------------------------------------------------------------+");
    println!("\n\n");
}
