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
    SelectColour,
    DateOkay,
    DateCancel,
    ColourOkay,
    ColourCancel,
    //Messages from child widgets
    TimezoneSelectChanged(i32, String),
    TimeSelectChanged(f64),
    TimezoneRemove(i32),
    //Messages to child widgets
    ChangeBaseTimezone(Option<String>),
    SetToNow,
}

pub struct App;

impl App {
    pub fn new() {
        Win::run(()).unwrap();
    }
}