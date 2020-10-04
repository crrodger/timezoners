use gtk::{Window, Box, BoxExt, Scale, Label, WidgetExt, OrientableExt, LabelExt, ComboBoxTextExt, ComboBoxTextBuilder,
    ButtonExt, PackType, Align, Orientation, ScaleExt, Inhibit, RangeExt, WidgetExt, };
use gtk::Orientation::{Horizontal, Vertical};
use relm::{Widget};
use relm_derive::{widget};
use chrono::{TimeZone, Utc};
use chrono_tz::{TZ_VARIANTS, Tz};

use self::Msg::*;

#[derive(Clone)]
pub struct Widgets {
    pub tz_box: Box,
    pub window: Window,
}

#[derive(Clone, Msg)]
pub enum Msg {
    TimeChange,
}
pub struct TzControlModel {
    utc_time: usize,
}



#[widget]
impl Widget for TzSelector {

    // fn me(&mut self) -> &mut TzSelector {
    //     return self;
    // }

    fn init_view(&mut self) {
        self.time_scale.set_range(0.0,96.0);
        self.time_scale.set_increments(1.0,4.0);
        
        self.cmb_tz_name.set_property_width_requested(50);
        for tz_name in TZ_VARIANTS.iter() {
            println!("Item {}", tz_name.name());
            self.cmb_tz_name.append_text(&tz_name.name());
        }

    }

    fn model() -> TzControlModel {
        TzControlModel {
            utc_time: 0,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            TimeChange => println!("Message"),
        }
    }

    view! {
        gtk::Box{
            orientation: Vertical,
            #[name="cmb_tz_name"]
            gtk::ComboBoxText {
                sensitive: true,
            },
            gtk::Box {
                orientation: Horizontal,
                gtk::Box {
                    orientation: Vertical,
                    vexpand: true,
                    gtk::Label {
                        label: "0:00 am",
                        widget_name: "time_min",
                        child: {
                            expand: false,
                            pack_type: PackType::Start,
                        }
                        // clicked => Increment,
                    },
                    gtk::Label {
                        label: "11:59 pm",
                        widget_name: "time_max",
                        child: {
                            expand: false,
                            pack_type: PackType::End,
                        }
                    },
                },
                #[name="time_scale"]
                gtk::Scale {
                    orientation: Vertical,
                    widget_name: "time_scale",
                    digits: 0,
                },
            }
        }
    }
}