use relm::{Widget};
// use gtk::prelude::*;
// use gtk::{Window, Builder, Type, 
//     MenuItem, Button, ButtonExt, Switch,
//     TreeView, TreeViewExt, ListStore, TreeModelFilter, TreeModelFilterExt, TreePath, TreeViewColumnBuilder, CellRendererTextBuilder, TreeModel, TreeIter,
// };
use crate::win::*;

#[derive(Debug)]
pub enum MsgUpdateType {
    StatusMessage,
}

#[derive(Msg)]
pub enum Msg {
    ProcessUpdateMsg((MsgUpdateType, String)),
    Quit,
}

pub struct App;

impl App {
    pub fn new() {
        Win::run(()).unwrap();
    }
}