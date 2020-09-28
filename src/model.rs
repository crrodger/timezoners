// use gtk::{ListStore, TreeModelFilter};
use relm::{Sender, Component};
use crate::widgets::*;

use crate::app::MsgUpdateType;

// #[derive(Debug)]
pub struct Model {
    pub tz_ctrls: Vec<Component<TzSelector>>,
    pub sender: Sender<(MsgUpdateType, String)>, 
}