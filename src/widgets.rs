use gtk::{Align, Box, BoxExt, ButtonExt, ComboBoxText, ComboBoxTextBuilder, ComboBoxTextExt, ComboBoxExt,  Inhibit, Label, LabelExt, OrientableExt, Orientation, PackType, RangeExt, Scale, ScaleExt, WidgetExt, Window};
use gtk::{Builder, Orientation::{Horizontal, Vertical}, prelude::BuilderExtManual, Adjustment};
use relm::{Update, Widget, Relm, EventStream};
use relm_derive::{widget};
use chrono::{TimeZone, Utc, NaiveDate, Local, Datelike};
use chrono_tz::{TZ_VARIANTS, Tz};
use crate::{win};

use self::Msg::*;

#[derive(Clone)]
pub struct MainWidgets {
    pub tz_box: Box,
    pub window: Window,
}

#[derive(Clone, Msg)]
pub enum Msg {
    TimezoneSelect,
    TimezoneSelectChanged(String),
    TimeSelect(f64),
    TimeSelectChanged(f64),
    BaseTimeSelectChanged(f64),
    BaseTimezoneChanged(String),
}
pub struct TzSelectorModel {
    base_timezone: Option<String>,
    this_timezone: Option<String>,
    local_relm: Relm<TzSelector>,
}

pub struct TzSelectorWidgets {
    pub box_root: Box,
    pub lbl_start: Label,
    pub lbl_end: Label,
    pub slider: Scale,
    pub cmb_tz_name: ComboBoxText,
    pub tz_scale_adj: Adjustment,
}


pub struct TzSelector {
    model: TzSelectorModel,
    widgets: TzSelectorWidgets,
}

impl TzSelector {

    fn updateTimeLabels(&self) {
        match &self.model.base_timezone {
            Some(base_zone) => {
                let local_now = Local::now();
                let base_start_time = NaiveDate::from_ymd(local_now.date().year(), local_now.month(), local_now.day()).and_hms(0, 0, 0);
                let base_end_time = NaiveDate::from_ymd(local_now.date().year(), local_now.month(), local_now.day()).and_hms(23, 59, 59);
                
                let tz_base: Tz = base_zone.parse().unwrap();
                let base_start_time_tz = tz_base.from_local_datetime(&base_start_time).unwrap();
                let base_end_time_tz = tz_base.from_local_datetime(&base_end_time).unwrap();

                let tz_curr: Tz;
                if let Some(tz) = self.model.this_timezone.clone() {
                    tz_curr = tz.parse().unwrap();
                } else {
                    return;
                }
                let curr_start_time_tz = base_start_time_tz.with_timezone(&tz_curr);
                let curr_end_time_tz = base_end_time_tz.with_timezone(&tz_curr);


                println!("Now is {} tz_base is {} tz_curr is {}",local_now, base_start_time_tz, curr_start_time_tz);

                self.widgets.lbl_start.set_text(format!("{}", curr_start_time_tz.format("%I:%M %P")).as_ref());
                self.widgets.lbl_end.set_text(format!("{}", curr_end_time_tz.format("%I:%M %P")).as_ref());

            },
            None => {

            },
            
        }

    }

    fn set_notification_channel() {

    }

}

impl Update for TzSelector {
    
    type Model = TzSelectorModel;
    type ModelParam = ();
    type Msg = Msg;
    
    fn update(&mut self, event: Msg) {
        match event {
            TimezoneSelect => {
                let tz_string = format!("{}", self.widgets.cmb_tz_name.get_active_text().unwrap());
                self.model.this_timezone = Some(tz_string.clone());
                self.updateTimeLabels();
                //Caught by parent win update loop
                self.model.local_relm.stream().emit(Msg::TimezoneSelectChanged(tz_string));
            },
            TimezoneSelectChanged(_new_zone) => {
                // Dummy, message is intercepted at win but have to complete match arms here
            },
            TimeSelect(value) => {
                // println!("Value {}", value);
                self.model.local_relm.stream().emit(Msg::TimeSelectChanged(value));
            },
            TimeSelectChanged(_new_value) => {
                // Dummy, message is intercepted at win but have to complete match arms here
            },
            BaseTimezoneChanged(new_zone) => {
                println!("Base tz change to {}", new_zone);
                self.model.base_timezone = Some(new_zone);
                self.updateTimeLabels();
            },
            BaseTimeSelectChanged(new_time) => {
                self.widgets.slider.set_value(new_time);
            },
        }
    }

    fn model(relm: &relm::Relm<Self>, param: Self::ModelParam) -> Self::Model {
        let local_relm = relm.clone();
        let base_timezone = None;
        let this_timezone = None;
        TzSelectorModel {
            base_timezone,
            this_timezone,
            local_relm,
        }
    }
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
        let cmb_tz_name: ComboBoxText = builder_widget.get_object("cmb_tz_name").expect("Could not get cmb_tz_name");
        let tz_scale_adj: Adjustment = builder_widget.get_object("tz_scale_adj").expect("Could not get tz_scale_adj");

        connect!(relm, cmb_tz_name, connect_changed(_), Msg::TimezoneSelect);
        connect!(relm, slider, connect_change_value(_, _, val), return (Some(Msg::TimeSelect(val)), Inhibit(false)));
        
        box_root.unparent();
        slider.set_adjustment(&tz_scale_adj);

        let widgets  = TzSelectorWidgets {
            box_root,
            lbl_start,
            lbl_end,
            slider,
            cmb_tz_name,
            tz_scale_adj,
        };

        TzSelector {
            model,
            widgets,
        }
    }

    fn init_view(&mut self) {
        
        for tz_name in TZ_VARIANTS.iter() {
            // println!("Item {}", tz_name.name());
            self.widgets.cmb_tz_name.append_text(&tz_name.name());
        }

    }

    
}

