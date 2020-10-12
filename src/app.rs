use relm::{Widget};
use crate::win::*;

#[derive(Debug)]
pub enum MsgUpdateType {
    StatusMessage,
}

#[derive(Msg)]
pub enum Msg {
    ProcessUpdateMsg((MsgUpdateType, String)),
    Quit,
    AddTzSelector(String),
    SelectDate,
    DateOkay,
    DateCancel,
    //Messages from child widgets
    TimezoneSelectChanged(i32, String),
    TimeSelectChanged(f64),
    TimezoneRemove(i32),
    //Messages to child widgets
    ChangeBaseTimezone(Option<String>),
}

pub struct App;

impl App {
    pub fn new() {
        Win::run(()).unwrap();
    }
}