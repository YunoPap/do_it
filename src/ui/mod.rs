pub mod main_menu;
pub mod task_view;

use ratatui::{Frame, layout::{Layout, Constraint}};
use crate::app::{AppState, CurrentScreen};

pub fn render(f: &mut Frame, app_state: &mut AppState) {
    let border_area = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .split(f.area())[0];
    
    match app_state.current_screen {
        CurrentScreen::MainMenu => main_menu::render_main_menu(f, app_state),
        CurrentScreen::TaskView => {
            if app_state.is_add_new {
                   crate::render_input_form(border_area, f, app_state);
               }else {
                   task_view::render_task_view(border_area, f, app_state);
               }
        }
        CurrentScreen::Themes => {},
        CurrentScreen::KeyBindings => {},
    }
}

