use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, ListItem, List},
    Frame,
};
use crate::app::AppState;

pub fn render_main_menu(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());
    
    let title = Paragraph::new(" === DO_IT! === ")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(title, chunks[0]);
    
    let menu_items = vec![
      ListItem::new("1. Start Program"),  
      ListItem::new("2. Themes (Blank)"),  
      ListItem::new("3. Key Bindings (Blank)"),  
      ListItem::new("4. Exit"),  
    ];
    
    let menu_list = List::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title(" Main Menu "));
    
    f.render_widget(menu_list, chunks[1]);
}