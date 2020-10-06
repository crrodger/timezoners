#![allow(dead_code)]

#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate chrono;
extern crate chrono_tz;

use crate::app::App;

mod app;
mod config;
mod win;
mod model;
mod widgets;

fn main() {
    App::new();
}