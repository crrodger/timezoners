// use gtk::{ListStore, TreeModelFilter};
use relm::{Sender, Component, Relm};
use chrono::{NaiveDate};
use crate::tzselector::*;
use serde::{Serialize, Deserialize};
use crate::{win::Win, app::MsgUpdateType};

// #[derive(Debug)]
pub struct Model {
    pub tz_ctrls: Vec<Component<TzSelector>>,
    pub tz_zones: Vec<Option<String>>,
    pub sender: Sender<(MsgUpdateType, String)>, 
    pub local_relm: Relm<Win>,
    pub base_tz: Option<String>,
    pub for_date: NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub zones:            Vec<Option<String>>,
    pub win_pos_x:        i32,
    pub win_pos_y:        i32,
    pub win_width:        i32,
    pub win_height:       i32,
}

//  If the content of this structure changes then delete config file from ~/Library/Preferences/<app-name> toml file
impl Default for Config {
    fn default() -> Self { 
        Self { 
            zones:              Vec::new(),
            win_pos_x:        0,
            win_pos_y:        0,
            win_width:        500,
            win_height:       300,
        }
    }
}