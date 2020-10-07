// use gtk::{ListStore, TreeModelFilter};
use relm::{Sender, Component, Relm};
use crate::widgets::*;

use crate::{win::Win, app::MsgUpdateType};

// #[derive(Debug)]
pub struct Model {
    pub tz_ctrls: Vec<Component<TzSelector>>,
    pub sender: Sender<(MsgUpdateType, String)>, 
    pub local_relm: Relm<Win>,
}