use chrono::{DateTime, Utc, TimeDelta, TimeZone};
use chrono_tz::Tz;
use ratatui::widgets::ListState;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentScreen {
    MainMenu,
    TaskView,
    Themes,
    KeyBindings,
}

#[derive(Debug)]
pub struct AppState {
    pub items: Vec<TodoItem>,
    pub list_state: ListState,
    pub is_add_new: bool,
    pub input_value: String,
    pub current_date: DateTime<Tz>,
    pub current_screen: CurrentScreen,
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
            current_screen: CurrentScreen::MainMenu,
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
pub struct TodoItem {
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
pub enum FormAction {
    None,
    Submit,
    Escape,
}
