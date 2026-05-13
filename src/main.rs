use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    prelude::Stylize,
    style::Color,
    widgets::{Block, BorderType, List, ListItem, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool, 
    description: String, 
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    state.items.push(TodoItem { 
        is_done: false, 
        description: String::from("Finish application"),
    });
    state.items.push(TodoItem { 
        is_done: false, 
        description: String::from("Finish application"),
    });
    state.items.push(TodoItem { 
        is_done: false, 
        description: String::from("Finish application"),
    });
    color_eyre::install()?;
    
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);
    
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state:&mut AppState) -> Result<()> {
    loop {
        //Rendering 
         terminal.draw(|f| render(f,app_state))?;
        //Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
        
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state:&mut AppState) {
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

    List::new(
        app_state 
        .items
        .iter()
        .map(|x| ListItem::from(x.description.clone()))
    )
    .render(inner_area, frame.buffer_mut());
}