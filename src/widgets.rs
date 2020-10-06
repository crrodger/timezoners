use glib::{ToValue, Type};
use gtk::{Align, Box, BoxExt, ButtonExt, ComboBox, ComboBoxExt, ComboBoxTextBuilder, Entry, Inhibit, Label, LabelExt, OrientableExt, Orientation, PackType, RangeExt, Scale, ScaleExt, TreeModelExt, WidgetExt, Window};
use gtk::{Builder, Orientation::{Horizontal, Vertical}, prelude::{GtkListStoreExtManual, BuilderExtManual}, Adjustment, 
            SearchEntry, SearchEntryExt, EntryExt, ListStore, TreeModelFilter, GtkListStoreExt, TreeViewColumnBuilder, CellRendererTextBuilder, CellLayoutExt, TreeModel, TreeIter, TreeModelFilterExt};
use relm::{Update, Widget, Relm, EventStream};
use relm_derive::{widget};
use chrono::{TimeZone, Utc, NaiveDate, NaiveTime, Local, Datelike, Duration};
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
    SetupModel,
    SearchContentsChange,
    LocalTimezoneSelect,
    NotifyParentTimezoneSelectChanged(String),
    LocalTimeSelect(f64),
    NotifyParentTimeSelectChanged(f64),
    FromParentBaseTimeSelectChanged(f64),
    FromParentBaseTimezoneChanged(String),
}
pub struct TzSelectorModel {
    base_timezone: Option<String>,
    this_timezone: Option<String>,
    local_relm: Relm<TzSelector>,
    pub liststore: ListStore,
    pub liststorefilter: TreeModelFilter,
}

pub struct TzSelectorWidgets {
    pub box_root: Box,
    pub lbl_start: Label,
    pub lbl_end: Label,
    pub slider: Scale,
    pub cmb_tz_name: ComboBox,
    pub tz_scale_adj: Adjustment,
    pub lbl_current_select_time: Label,
    pub txt_search_tz: SearchEntry,
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
                    //If no timezone is selected for current control do nothing
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

    fn update_time_display(&self) {
        let slider_value = self.widgets.slider.get_value();
        let display_value = get_time_string_from_index(slider_value, self.widgets.lbl_start.get_text().as_str());
        self.widgets.lbl_current_select_time.set_text(&display_value);
    }

    fn setup_model(&self) {
        let new_cell = CellRendererTextBuilder::new();
        let cell = new_cell.build();
        self.widgets.cmb_tz_name.pack_start(&cell, true);
        self.widgets.cmb_tz_name.add_attribute(&cell, "text", 0);
        self.widgets.cmb_tz_name.set_id_column(0);
        
    }

    fn add_timezone_strings(&self) {
        for tz_name in TZ_VARIANTS.iter() {
            // println!("Item {}", tz_name.name());
            self.add_data_row(&tz_name.name(), &tz_name.name());
        }
    }

    fn add_data_row(&self, col1: &str, col2: &str) {
        let row = self.model.liststore.append();

        self.model.liststore.set_value(&row, 0, &col1.to_value());
        // self.model.liststore.set_value(&row, 1, &col2.to_value());
    }

    fn add_text_column(&self, title: &str, column: i32) {
        let mut new_column = TreeViewColumnBuilder::new();
        new_column = new_column.resizable(true);
        new_column = new_column.reorderable(true);
        new_column = new_column.title(title);

        let new_cell = CellRendererTextBuilder::new();
        
        // let view_column = TreeViewColumn::new();
        let view_column = new_column.build();
        let cell = new_cell.build();
        view_column.pack_start(&cell, true);
        view_column.add_attribute(&cell, "text", column);
        self.widgets.cmb_tz_name.set_id_column(0);
    }
    
}

fn get_time_string_from_index(value: f64, start_time: &str) ->String {
    let starting_time: NaiveTime = NaiveTime::parse_from_str(start_time, "%I:%M %P").unwrap();
    let offset_dur: Duration = Duration::minutes(value as i64 * 15);
    let calc_time = starting_time + offset_dur;
    let ret_string = String::from(format!("{}", calc_time.format("%I:%M %P")));
    return ret_string;
}

impl Update for TzSelector {
    
    type Model = TzSelectorModel;
    type ModelParam = ();
    type Msg = Msg;
    
    fn update(&mut self, event: Msg) {
        match event {
            SetupModel => {
                self.setup_model();
                self.add_timezone_strings();
            },
            SearchContentsChange => {
                if self.widgets.txt_search_tz.get_text_length() >= 3 {
                    self.model.liststorefilter.refilter();
                    self.widgets.cmb_tz_name.popup();
                }
                
            },
            LocalTimezoneSelect => {
                let tz_string: String;
                if let Some(sel_str) = self.widgets.cmb_tz_name.get_active_id() {
                    tz_string = String::from(sel_str.as_str());
                } else {
                    return;
                }
                // let tz_string = format!("{}", self.widgets.cmb_tz_name.get_active_id().unwrap());
                self.model.this_timezone = Some(tz_string.clone());
                self.updateTimeLabels();
                //Caught by parent win update loop
                self.model.local_relm.stream().emit(Msg::NotifyParentTimezoneSelectChanged(tz_string));
            },
            LocalTimeSelect(value) => {
                // println!("Value {}", value);
                self.model.local_relm.stream().emit(Msg::NotifyParentTimeSelectChanged(value));
                self.update_time_display();
            },
            NotifyParentTimezoneSelectChanged(_new_zone) => {
                // Dummy, message is intercepted at win but have to complete match arms here
            },
            NotifyParentTimeSelectChanged(_new_value) => {
                // Dummy, message is intercepted at win but have to complete match arms here
            },
            FromParentBaseTimezoneChanged(new_zone) => {
                println!("Base tz change to {}", new_zone);
                self.model.base_timezone = Some(new_zone);
                self.updateTimeLabels();
            },
            // Should only be recieved by non base timezone Tz Controls
            FromParentBaseTimeSelectChanged(new_time) => {
                self.widgets.slider.set_value(new_time);
                self.update_time_display();
            },
        }
    }

    fn model(relm: &relm::Relm<Self>, param: Self::ModelParam) -> Self::Model {
        let local_relm = relm.clone();
        let base_timezone = None;
        let this_timezone = None;
        let liststore = ListStore::new(&[
            Type::String,
        ]);
        
        let liststorefilter = TreeModelFilter::new(&liststore, None); //Probably need a TreePath for a tree not a list like I am using here

        TzSelectorModel {
            base_timezone,
            this_timezone,
            local_relm,
            liststore,
            liststorefilter,
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
        let cmb_tz_name: ComboBox = builder_widget.get_object("cmb_tz_name").expect("Could not get cmb_tz_name");
        let tz_scale_adj: Adjustment = builder_widget.get_object("tz_scale_adj").expect("Could not get tz_scale_adj");
        let lbl_current_select_time: Label = builder_widget.get_object("lbl_current_select_time").expect("Could not get lbl_current_select_time");
        let txt_search_tz: SearchEntry = builder_widget.get_object("txt_search_tz").expect("Could not get txt_search_tz");

        connect!(relm, cmb_tz_name, connect_changed(_), Msg::LocalTimezoneSelect);
        connect!(relm, slider, connect_change_value(_, _, val), return (Some(Msg::LocalTimeSelect(val)), Inhibit(false)));
        connect!(relm, txt_search_tz, connect_search_changed(_), Msg::SearchContentsChange);
        relm.stream().emit(Msg::SetupModel);
        
        cmb_tz_name.set_model(Some(&model.liststorefilter));

        let clone_search = txt_search_tz.clone();
        model.liststorefilter.set_visible_func(move |tm: &TreeModel, ti: &TreeIter| {
            if clone_search.get_text_length() > 0 {
                
                let match_chars = clone_search.get_text();

                match tm.get_value(ti, 0).get::<String>().unwrap() {
                    Some(str_col_value) => {
                        if str_col_value.contains(match_chars.as_str()) {
                                return true;
                            } else {
                                return false;
                            }
                    },
                    None => return true
                }
            } else {
                true
            }
        });

        //The component is loaded inside of a window, need to remove this link
        box_root.unparent();
        slider.set_adjustment(&tz_scale_adj);

        let widgets  = TzSelectorWidgets {
            box_root,
            lbl_start,
            lbl_end,
            slider,
            cmb_tz_name,
            tz_scale_adj,
            lbl_current_select_time,
            txt_search_tz,
        };

        TzSelector {
            model,
            widgets,
        }
    }

    fn init_view(&mut self) {
        
    }
    
}

