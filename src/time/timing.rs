use std::ops::{Add, Mul, Sub};
#[derive(Debug, Copy, Clone)]
pub struct Time {
    pub minute: u8,
    pub hour: u8,
}
impl Add for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        let total_minutes = self.minute as u16 + other.minute as u16;
        let additional_hour = total_minutes / 60;
        let new_minutes = total_minutes % 60;

        let total_hours = self.hour as u16 + other.hour as u16 + additional_hour;
        let new_hours = total_hours % 24;

        Time {
            minute: new_minutes as u8,
            hour: new_hours as u8,
        }
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        let self_minutes = self.hour as i32 * 60 + self.minute as i32;
        let other_minutes = other.hour as i32 * 60 + other.minute as i32;

        let diff_minutes = (self_minutes - other_minutes + 24 * 60) % (24 * 60);

        Time {
            hour: (diff_minutes / 60) as u8,
            minute: (diff_minutes % 60) as u8,
        }
    }
}

impl Mul<u8> for Time {
    type Output = Time;

    fn mul(self, rhs: u8) -> Self::Output {
        let total_minutes = (self.hour as u32 * 60 + self.minute as u32) * rhs as u32;
        let new_hours = (total_minutes / 60) % 24;
        let new_minutes = total_minutes % 60;

        Time {
            hour: new_hours as u8,
            minute: new_minutes as u8,
        }
    }
}

impl Time {
    pub fn from_string(input: String) -> Self {
        if input.as_str().contains(":") {
            Time {
                minute: input
                    .as_str()
                    .split(":")
                    .nth(1)
                    .expect("couldn't get first element")
                    .parse::<u8>()
                    .expect("couldn't read number"),
                hour: input
                    .as_str()
                    .split(":")
                    .nth(0)
                    .expect("couldn't get first element")
                    .parse::<u8>()
                    .expect("couldn't convert to u8"),
            }
        } else {
            Time {
                minute: 0,
                hour: input.parse::<u8>().expect("couldn't read number"),
            }
        }
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let width = f.width().unwrap_or(0);

        let time_str = format!("{:02}:{:02}", self.hour, self.minute);

        match f.align() {
            Some(std::fmt::Alignment::Left) => write!(f, "{:<width$}", time_str, width = width),
            Some(std::fmt::Alignment::Right) => write!(f, "{:>width$}", time_str, width = width),
            Some(std::fmt::Alignment::Center) => write!(f, "{:^width$}", time_str, width = width),
            None => write!(f, "{}", time_str),
        }
    }
}