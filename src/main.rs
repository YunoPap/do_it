pub mod app;
pub mod ui;

use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame, 
    crossterm::event::{self, Event, KeyEvent}, 
    prelude::Stylize, style::{Color}, text::ToSpan, 
    widgets::{Block, BorderType, Padding, Paragraph, Widget}
};
use std::fs::File;
use std::io::{Read, Write};
use app::{AppState, TodoItem, CurrentScreen, FormAction};


const SAVE_FILE: &str = "todos.json";

// Entry point of the application
fn main() -> Result<()> {
    let mut app_state = app::AppState::default();
    app_state.is_add_new = false;
     
    // Save app_state
    app_state.items = load_todos();
    
    if !app_state.items.is_empty() {
        app_state.list_state.select(Some(0));
    }
    
    color_eyre::install()?;
    
    let terminal = ratatui::init();
    let result = run(terminal, &mut app_state);
    
    ratatui::restore();
    
    if result.is_ok() {
        save_todos(&app_state.items)?;
    }
    result
}

// Main application loop
fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| ui::render(f, app_state))?;
        // Input handling
        if let Event::Key(key) = event::read()? {
            if app_state.current_screen == CurrentScreen::MainMenu {
                match key.code {
                    event::KeyCode::Char('1') => app_state.current_screen = CurrentScreen::TaskView,
                    event::KeyCode::Char('4') |  event::KeyCode::Esc => break,
                    _=> {}
                }
            } else if app_state.is_add_new {
                match handle_add_new(key, app_state) {
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
            } else {
                if handle_key(key, app_state) {
                    app_state.current_screen = CurrentScreen::MainMenu;
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

// Render the UI based on the current app_state


// Render the list of to-do items
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