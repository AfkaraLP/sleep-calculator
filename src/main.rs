// the crate for my own time format
mod time;

// used to get the local time
use chrono::Local;

// pattern matching
use regex::Regex;

use time::timing::Time;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Sleep Calculator",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    desired_time: String,
}

impl MyApp {
    fn is_valid_time_format(&self) -> bool {
        // chatgpt made this idfk how regex works
        let re = Regex::new(r"^([01]?\d|2[0-3]):[0-5]\d$|^([01]?\d|2[0-3])$").unwrap();
        re.is_match(&self.desired_time)
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            desired_time: "00:00".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sleep Calculator");

            ui.horizontal(|ui| {
                let something_label = ui.label("Your Desired Time To Wake Up");
                ui.text_edit_singleline(&mut self.desired_time)
                    .labelled_by(something_label.id)
            });

            ui.label(format!(
                "current time is: {}",
                Local::now().format("%H:%M").to_string()
            ));

            let local_time = Local::now().format("%H:%M").to_string();

            ui.heading("If you go to bed right now");

            for sleep_cycle in 1..10 {
                let time_to_add = Time {
                    hour: 1,
                    minute: 30,
                } * sleep_cycle as u8
                    + Time {
                        hour: 0,
                        minute: 14,
                    };

                let time_to_wake_up = Time::from_string(local_time.clone()) + time_to_add;

                let message = format!(
                    "if you go to bed right now you can wake up at {} for {} sleep cycles",
                    time_to_wake_up, sleep_cycle
                );

                ui.label(message);
            }

            ui.heading("to wake up at your desired time");

            if self.is_valid_time_format() {
                let desired_time = Time::from_string(self.desired_time.clone());

                for sleep_cycle in 1..10 {
                    let time_to_subtract = Time {
                        hour: 1,
                        minute: 30,
                    } * sleep_cycle as u8;

                    let time_to_fall_asleep = desired_time - time_to_subtract;

                    let time_to_fall_asleep =
                        time_to_fall_asleep - Time::from_string("00:14".to_string());
                    let time_till_fall_asleep =
                        time_to_fall_asleep - Time::from_string(local_time.clone());

                    if time_till_fall_asleep.hour > 12 {
                        continue;
                    }

                    let message = format!(
                        "for {} cycles of sleep you will have to go to bed at {} which is in {}",
                        sleep_cycle, time_to_fall_asleep, time_till_fall_asleep
                    );

                    ui.label(message);
                }
            }
        });
    }
}
