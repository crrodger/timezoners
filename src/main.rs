#![allow(dead_code)]

#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate relm_test;
extern crate chrono;
extern crate chrono_tz;
extern crate gdk;

use crate::app::App;

mod app;
mod config;
mod win;
mod model;
mod widgets;
mod tzselector;

fn main() {
    App::new();
}
