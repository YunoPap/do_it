use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event::{self, Event}, 
    widgets::{Paragraph, Widget},
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
    Paragraph::new("Hello from application").render(frame.area(), frame.buffer_mut());
}