use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal,
};

use crate::utils::terminal::refresh_display;

pub struct Confirm {
    message: String,
}

impl Confirm {
    pub fn new() -> Self {
        Confirm {
            message: String::new(),
        }
    }

    pub fn set_message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    pub fn ask(&self) -> bool {
        println!("{}", self.message);
        println!("Press enter to confirm or c to cancel");

        loop {
            terminal::enable_raw_mode().expect("Failed to enable raw mode");

            let event = read().unwrap();
            match event {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Char('q') => {
                        terminal::disable_raw_mode().unwrap();
                        std::process::exit(0);
                    }
                    KeyCode::Char('c') => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        refresh_display(2);
                        return false;
                    }
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        refresh_display(2);
                        return true;
                    }
                    _ => {}
                },
                _ => {}
            }
            terminal::disable_raw_mode().expect("Failed to disable raw mode");
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OptionSelect {
    title: String,
    options: Vec<String>,
}

impl OptionSelect {
    pub fn new() -> Self {
        OptionSelect {
            title: String::new(),
            options: Vec::new(),
        }
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn add_option(mut self, option: String) -> Self {
        self.options.push(option);
        self
    }

    pub fn add_option_if_true(mut self, option: String, condition: bool) -> Self {
        if condition {
            self.options.push(option);
        }
        self
    }

    pub fn ask(&self) -> String {
        let mut current_option = 0;
        println!("{}:", self.title);

        loop {
            for (i, option) in self.options.iter().enumerate() {
                if i == current_option {
                    println!("> [{}] {}", i + 1, option);
                    continue;
                }
                println!("  [{}] {}", i + 1, option);
            }
            terminal::enable_raw_mode().expect("Failed to enable raw mode");

            let event = read().unwrap();
            match event {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Char('q') => {
                        terminal::disable_raw_mode().unwrap();
                        std::process::exit(0);
                    }
                    KeyCode::Up => {
                        if current_option > 0 {
                            current_option -= 1;
                        } else {
                            current_option = self.options.len() - 1;
                        }
                    }
                    KeyCode::Down => {
                        if current_option < self.options.len() - 1 {
                            current_option += 1;
                        } else {
                            current_option = 0;
                        }
                    }
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        return self.options[current_option].to_string();
                    }
                    _ => {}
                },
                _ => {}
            }
            terminal::disable_raw_mode().expect("Failed to disable raw mode");
            refresh_display(self.options.len() as u16);
        }
    }
}
