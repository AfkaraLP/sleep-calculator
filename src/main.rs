mod time;

use chrono::Local;
use std::env::args;

use time::timing::Time;

fn main() {
    let local_time = Local::now();
    let local_time = local_time.format("%H:%M").to_string();
    let local_time: Time = Time::from_string(local_time);

    let argument = args().nth(1).expect("no argument provided");
    let desired_time = Time::from_string(argument);

    println!("\n\n");
    println!("+-----------------------------------------------------------------+");
    println!(
        "| if you want to wake up at {:<6}                                |",
        desired_time
    );
    println!("+-----------------------------------------------------------------+");
    for sleep_cycle in 1..10 {
        let time_to_subtract = Time {
            hour: 1,
            minute: 30,
        } * (sleep_cycle as u8);

        let time_to_fall_asleep = desired_time - time_to_subtract;
        let time_to_fall_asleep = time_to_fall_asleep
            - Time {
                hour: 0,
                minute: 14,
            };
        let time_till_fall_asleep = time_to_fall_asleep - local_time;
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
