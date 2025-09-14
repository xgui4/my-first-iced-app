use std::{fs, path::PathBuf, vec};

use iced::{widget::{button, column, row, text}, window, Element, Length::Fill, Task};

#[derive(Debug)]
struct AppState {
    current_dir: PathBuf,
    current_files: Vec<(String, bool)>
}

impl Default for AppState  {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap(); 
        let current_files = get_files(&current_dir); 
        AppState { 
            current_dir,
            current_files
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Exit
}

fn update(state : &mut AppState, message:Message) -> Task<Message> {
    match message {
        Message::Exit => window::get_latest().and_then(window::close),
    }
}

fn view(state : &AppState) -> Element<Message> {
    let mut content = column![row![        
        text(state.current_dir.to_str().unwrap_or("Unknow Directory"))
        .size(32).width(Fill),
        button(text("Up").size(24)).on_press(Message::Exit),
        button(text("Exit").size(24)).on_press(Message::Exit)
    ].spacing(8)]; 

    for file in &state.current_files {
        content = content.push(text(&file.0))
    }

   content.into()
}

fn main() -> iced::Result  {
    iced::application("Rusted Window", update, view).theme(|_s| iced::Theme::CatppuccinMocha).run()
}

fn get_files(path: &PathBuf) -> Vec<(String, bool)> {
    let mut dirs = Vec::default();
    let mut files = Vec::default(); 

    if let Ok(read_dir) = fs::read_dir(path) {
        for read in read_dir {
            if let Ok(dir_entry) = read {
                if let Some(name) = dir_entry.file_name().to_str() {
                    if dir_entry.path().is_dir() {
                        dirs.push((name.to_string(), true))
                    } 
                    else {
                        files.push((name.to_string(), true));
                    }
                }
            }
        } 
    }

    dirs.append(&mut files);
    dirs
}

// Source : https://youtu.be/2CQ4hLB2IMw?si=AivLczXcm-ryd2eG&t=1125