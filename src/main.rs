use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyEvent},
    DefaultTerminal,
    Frame,
    layout::{Constraint, Layout},
    prelude::Stylize,
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool, 
    description: String, 
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    state.is_add_new = false;

    state.items.push(TodoItem { 
        is_done: false, 
        description: String::from("Description 1"),
    });
    state.items.push(TodoItem { 
        is_done: false, 
        description: String::from("Description 2"),
    });
    state.items.push(TodoItem { 
        is_done: false, 
        description: String::from("Description 3"),
    });
    color_eyre::install()?;
    
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);
    
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| render(f, app_state))?;
        // Input handling
        if let Event::Key(key) = event::read()? {
            if app_state.is_add_new {
                if handle_add_new(key, app_state) {
                    app_state.is_add_new = false;
            }

            }else{
                if handle_key(key, app_state) {
                    break;
                }
            }
            
        }
    }

    Ok(())
}

fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Enter => {
            return true;
        }
        event::KeyCode::Esc => {
            return true;
        }
        _ => {}
    }
    false
}

fn handle_key(key: event::KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => true,
        event::KeyCode::Char(c) => {
            match c {
                'A' => {
                    app_state.is_add_new = true;
                }
                'D' => {
                    if let Some(index) = app_state.list_state.selected() {
                        app_state.items.remove(index);
                    }
                }
                'k' => {
                    app_state.list_state.select_previous();
                }
                'j' => {
                    app_state.list_state.select_next();
                }
                _ => {}
            }
            false
        }
        _ => false,
    }
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let border_area = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .split(frame.area())[0];
    
    let inner_area = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .split(border_area)[0];

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    let list = List::new(
        app_state 
        .items
        .iter()
        .map(|x| ListItem::from(x.description.clone()))
    )
    .highlight_symbol(">")
    .highlight_style(Style::default().fg(Color::Green));
    
    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);


    if app_state.is_add_new {
        Paragraph::new("Hello from application").render(frame.area(), frame.buffer_mut());
    }
}