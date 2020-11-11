use std::{time::Duration, thread};

use chrono::{DateTime, Local};

#[derive(Copy, Clone)]
struct Column {
    pub rows: [bool; 4],
}

impl Column {
    fn new() -> Column {
        Column {
            rows: [false; 4]
        }
    }

    fn update(&mut self, mut digit: i32) {
        for i in 0..4 {
            let index: usize = (3-i) as usize;
            let binary_number = 2i32.pow(index as u32);

            if digit - binary_number >= 0 {
                self.rows[index] = true;
                digit -= binary_number;
            } else {
                self.rows[index] = false;
            }
        }
    }
}

pub trait Clock {
    fn update(&mut self);
    fn draw(&self);
}

struct BinaryClock {
    formatted: String,
    columns: [Column; 6],
}

impl BinaryClock {
    fn new() -> BinaryClock {
        BinaryClock {
            formatted: Local::now().format("%H%M%S").to_string(),
            columns: [Column::new(); 6],
        }
    }
}

impl Clock for BinaryClock {
    fn update(&mut self) {
        self.formatted = Local::now().format("%H%M%S").to_string();

        for (index, _) in self.formatted.chars().enumerate() {
            let digit = &self.formatted[index..(index+1)];
            self.columns[index].update(digit.parse().unwrap());
            self.columns[index].rows.reverse();
        }
    }

    fn draw(&self) {
        println!("     HH  MM  SS");
        for row in 0..4 {
            let mut row_display: String = String::new();

            let binary_number = 2i32.pow(3-row);
            let binary_number_string = format!("{:02}:  ", binary_number.to_string());
            row_display.push_str(&binary_number_string[..]);

            for col in 0..6 {
                if self.columns[col].rows[row as usize] {
                    row_display.push_str("x");
                } else {
                    row_display.push_str(".");
                }

                if (col+1) % 2 == 0 {
                    row_display.push_str("  ");
                }
            }
            println!("{:}", row_display);
            row_display = String::new();
        }
    }
}

fn main() {
    let mut clock = BinaryClock::new();

    loop {
        // Clear screen.
        println!("\x1b[2J");

        clock.update();
        clock.draw();

        thread::sleep(Duration::from_millis(1000))
    }
}
