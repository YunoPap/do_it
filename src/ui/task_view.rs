use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, Widget},
    prelude::Stylize
};
use crate::app::AppState;


pub fn render_task_view(border_area: Rect, frame: &mut Frame<'_>, app_state: &mut AppState) {
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
