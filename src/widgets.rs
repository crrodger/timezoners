use gtk::{Align, Box, BoxExt, ButtonExt, ComboBoxText, ComboBoxTextBuilder, ComboBoxTextExt, Inhibit, Label, LabelExt, OrientableExt, Orientation, PackType, RangeExt, Scale, ScaleExt, WidgetExt, Window};
use gtk::{Builder, Orientation::{Horizontal, Vertical}, prelude::BuilderExtManual};
use relm::{Update, Widget, Relm};
use relm_derive::{widget};
use chrono::{TimeZone, Utc};
use chrono_tz::{TZ_VARIANTS, Tz};

use self::TzMsg::*;

#[derive(Clone)]
pub struct MainWidgets {
    pub tz_box: Box,
    pub window: Window,
}

#[derive(Clone, Msg)]
pub enum TzMsg {
    TimeChange,
}
pub struct TzSelectorModel {
    utc_time: usize,
}

pub struct TzSelectorWidgets {
    pub box_root: Box,
    pub lbl_start: Label,
    pub lbl_end: Label,
    pub slider: Scale,
    pub cmb_tz_name: ComboBoxText,
}


pub struct TzSelector {
    model: TzSelectorModel,
    widgets: TzSelectorWidgets,
}

impl TzSelector {

}

impl Widget for TzSelector {
    // Specify the type of the root widget.
    type Root = Box;
    
    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.widgets.box_root.clone()
    }

    fn view(relm: &Relm<Self>, mut model: Self::Model) -> Self {
        let glade_src_widg = include_str!("timezoners_tz_widget.glade");
        let builder_widget = Builder::from_string(glade_src_widg);

        let box_root: Box = builder_widget.get_object("box_widget_main").expect("Could not get box_widget_main");
        let lbl_start: Label = builder_widget.get_object("tz_label_start").expect("Could not get tz_label_start");
        let lbl_end: Label = builder_widget.get_object("tz_label_end").expect("Could not get tz_label_end");
        let slider: Scale = builder_widget.get_object("tz_scale_select").expect("Could not get tz_scale_select");
        let cmb_tz_name = builder_widget.get_object("cmb_tz_name").expect("Could not get cmb_tz_name");

        box_root.unparent();

        let widgets  = TzSelectorWidgets {
            box_root,
            lbl_start,
            lbl_end,
            slider,
            cmb_tz_name,
        };

        TzSelector {
            model,
            widgets,
        }
    }

    fn init_view(&mut self) {
        
        for tz_name in TZ_VARIANTS.iter() {
            println!("Item {}", tz_name.name());
            self.widgets.cmb_tz_name.append_text(&tz_name.name());
        }

    }

    
}

impl Update for TzSelector {
    
    type Model = TzSelectorModel;
    type ModelParam = ();
    type Msg = TzMsg;
    
    fn update(&mut self, event: TzMsg) {
        match event {
            TimeChange => println!("Message"),
        }
    }

    fn model(relm: &relm::Relm<Self>, param: Self::ModelParam) -> Self::Model {
        TzSelectorModel {
            utc_time: 0,
        }
    }
}