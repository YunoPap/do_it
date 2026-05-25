use color_eyre::eyre::Result;
use ratatui::crossterm::style::ContentStyle;
use ratatui::{
    DefaultTerminal, Frame, crossterm::event::{self, Event, KeyEvent}, layout::{Constraint, Layout}, prelude::Stylize, style::{Color, Style}, text::ToSpan, widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget}
};
use serde::{Deserialize, Serialize, Deserializer, Serializer};
use std::fs::File;
use std::io::{Read, Write};
use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use chrono_tz::Tz;

// Represents the overall state of the 
// application, including the list of to-do 
// items, the current selection, and whether the input form is active
#[derive(Debug)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
    input_value: String,
    current_date: DateTime<Tz>,
}

impl Default for AppState {
    fn default() -> Self {
        // Time Zone
        let est = Tz::America__New_York;
        let current_time = Utc::now().with_timezone(&est);
      
        Self {
            items: Vec::new(),
            list_state: ListState::default(),
            is_add_new: false,
            input_value: String::new(),
            current_date: current_time, 
        }
    }
}

impl AppState {
    pub fn next_day(&mut self) {
        self.current_date = self.current_date + TimeDelta::days(1);
    }
    pub fn prev_day(&mut self) {
        self.current_date = self.current_date + TimeDelta::days(-1);
    }
}
// Represents a single to-do item with its description and completion status
#[derive(Debug, Serialize, Deserialize)]
struct TodoItem {
    pub is_done: bool, 
    pub description: String, 
    
    #[serde(with="ts_rw")]
    pub date: DateTime<Tz>,
}
mod ts_rw {
    use super::*;

    // 1. How to SAVE the date to JSON
    pub fn serialize<S>(date: &DateTime<Tz>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert to a regular UTC timestamp number
        serializer.serialize_i64(date.timestamp_millis())
    }

    // 2. How to LOAD the date from JSON
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Tz>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = i64::deserialize(deserializer)?;
        
        // Hardcode your default system timezone format to match your AppState
        let tz = Tz::America__New_York; 
        
        // Reconstruct the DateTime wrapper using the timezone
        let datetime_utc = Utc.timestamp_millis_opt(millis)
            .single()
            .ok_or_else(|| serde::de::Error::custom("Invalid timestamp"))?;
            
        Ok(datetime_utc.with_timezone(&tz))
    }
}

// Represents the possible actions that can be taken in the input form
enum FormAction {
    None,
    Submit,
    Escape,
}

const SAVE_FILE: &str = "todos.json";

// Entry point of the application
fn main() -> Result<()> {
    let mut state = AppState::default();
    state.is_add_new = false;
     
    // Save State
    state.items = load_todos();
    
    if !state.items.is_empty() {
        state.list_state.select(Some(0));
    }
    
    color_eyre::install()?;
    
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);
    
    ratatui::restore();
    
    if result.is_ok() {
        save_todos(&state.items)?;
    }
    result
}

// Main application loop
fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| render(f, app_state))?;
        // Input handling
        if let Event::Key(key) = event::read()? {
            if app_state.is_add_new {
                match handle_add_new(key, app_state){
                    FormAction::None => {},
                    FormAction::Submit => {
                        app_state.items.push(TodoItem {
                            is_done: false,
                            description: app_state.input_value.clone(),
                            date: app_state.current_date,
                        });
                        app_state.input_value.clear();
                        app_state.is_add_new = false;
                    },
                    FormAction::Escape => {
                        app_state.is_add_new = false;
                        app_state.input_value.clear();
                    }
                }
            } else{
                if handle_key(key, app_state) {
                    break;
                }
            }
            
        }
    }

    Ok(())
}

// Handle input when adding a new item
fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> FormAction {
    match key.code {
        event::KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }
        event::KeyCode::Backspace => {
            app_state.input_value.pop();
        }
        event::KeyCode::Enter => {
            return FormAction::Submit;
        }
        event::KeyCode::Esc => {
            return FormAction::Escape;
        }
        _ => {}
    }
    FormAction::None
}

// Handle input when navigating the list or toggling items
fn handle_key(key: event::KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Enter => {
            if let Some(index) = app_state.list_state.selected(){
                if let Some(item) = app_state.items.get_mut(index) {
                    item.is_done = !item.is_done;
                }
            }
            false
        }
        event::KeyCode::Esc => true,
        event::KeyCode::Char(c) => {
            match c {
                'A' => {
                    app_state.is_add_new = true;
                }
                'D' => {
                    if let Some(index) = app_state.list_state.selected() {
                        app_state.items.remove(index);
                        
                        if app_state.items.is_empty() {
                            app_state.list_state.select(None);
                        } else if index >= app_state.items.len() {
                            app_state.list_state.select(Some(app_state.items.len() - 1));
                        }
                    }
                }
                'k' => {
                    app_state.list_state.select_previous();
                }
                'j' => {
                    app_state.list_state.select_next();
                }
                'h' => {
                    app_state.prev_day();
                }
                'l' => {
                    app_state.next_day();
                }
                _ => {}
            }
            false
        }
        _ => false,
    }
}

// Render the UI based on the current state
fn render(frame: &mut Frame, app_state: &mut AppState) {
    let border_area = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .split(frame.area())[0];

    if app_state.is_add_new {
        render_input_form(border_area, frame, app_state);
    }else {
        render_list(border_area, frame, app_state);
    }
}

// Render the list of to-do items
fn render_list(
    border_area: ratatui::prelude::Rect,
    frame: &mut Frame<'_>,
    app_state: &mut AppState,
) {
    let the_time = app_state.current_date.format("%Y-%m-%d").to_string();
    
    let inner_area = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .split(border_area)[0];

    Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Do_It!".to_span().into_left_aligned_line())
        .title(the_time.to_span().into_right_aligned_line())
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    let list = List::new(app_state.items.iter()
        .filter(|item| {
            let item_day = item.date.date_naive();
            let current_view_day = app_state.current_date.date_naive();
            let real_today = chrono::Utc::now().with_timezone(&chrono_tz::Tz::America__New_York).date_naive();
            
            let is_same_day = item_day == current_view_day;
            let is_past_incomplete = !item.is_done && item_day < current_view_day;
            let is_viewing_future = current_view_day > real_today;
            
            is_same_day || (is_past_incomplete && !is_viewing_future)
        })
        .map(|x| {
        let value = if x.is_done {
            x.description.to_span().crossed_out()
        } else {
            x.description.to_span()
             };
             ListItem::from(value)
        }))
    .highlight_symbol(">")
    .highlight_style(Style::default().fg(Color::Green));
    
    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
}

// Render the input form for adding a new to-do item
fn render_input_form (
    border_area: ratatui::prelude::Rect,
    frame: &mut Frame<'_>,
    app_state: &mut AppState,
) {
    Paragraph::new(app_state.input_value.as_str())
        .block(
            Block::bordered()
                .title(" Input Description ".to_span().into_left_aligned_line())
                .fg(Color::Green)
                .padding(Padding::uniform(1))
                .border_type(BorderType::Rounded)
        )
        .render(border_area, frame.buffer_mut());
}

fn load_todos() -> Vec<TodoItem> {
    let mut file = match File::open(SAVE_FILE) {
        Ok(f) => f,
        Err(_) => return Vec::new()
    };
    
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_ok() {
        serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
} 

fn save_todos(items: &[TodoItem]) -> Result<()> {
    let mut file = File::create(SAVE_FILE)?;
    let json = serde_json::to_string_pretty(items)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}