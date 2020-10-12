use glib::{ToValue, Type};
use gtk::{Box, ToolButton, Button, ButtonExt, ComboBox, ComboBoxExt, Inhibit, Label, LabelExt, RangeExt, Scale, TreeModelExt, WidgetExt, Window};
use gtk::{Builder, prelude::{GtkListStoreExtManual, BuilderExtManual}, Adjustment, DrawingArea,
            SearchEntry, SearchEntryExt, EntryExt, ListStore, TreeModelFilter, GtkListStoreExt, TreeViewColumnBuilder, CellRendererTextBuilder, 
            CellLayoutExt, TreeModel, TreeIter, TreeModelFilterExt, CssProvider, CssProviderExt, STYLE_PROVIDER_PRIORITY_APPLICATION, StyleContextExt};
use relm::{Update, Widget, Relm, DrawHandler};
use gdk::{EventKey};
use cairo::{LinearGradient, Pattern};
use chrono::{TimeZone, NaiveDate, NaiveTime, Local, Datelike, Timelike, Duration, DateTime};
use chrono_tz::{TZ_VARIANTS, Tz};

use self::Msg::*;

#[derive(Clone)]
pub struct MainWidgets {
    pub tz_box: Box,
    pub window: Window,
    pub tb_btn_add_tz: ToolButton,
}

#[derive(Clone, Msg)]
pub enum Msg {
    SearchContentsChange,
    SearchKeyReleased(EventKey),
    SearchLostFocus,
    DrawIllumination,
    RemoveTz,
    LocalTimezoneSelect,
    NotifyParentTimezoneSelectChanged(i32, String),
    LocalTimeSelect(f64),
    NotifyParentTimeSelectChanged(f64),
    NotifyParentBaseTzChanged(String),
    NotifyParentTzSelectorRemoveClicked(i32),
    FromParentBaseTimeSelectChanged(f64),
    FromParentBaseTimezoneChanged(Option<String>),
}
pub struct TzSelectorModel {
    index: i32,
    base_timezone: Option<String>,
    this_timezone: Option<String>,
    local_relm: Relm<TzSelector>,
    draw_handler: DrawHandler<DrawingArea>,
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
    pub pb_remove_tz: Button,
    pub draw_illum: DrawingArea,
}


pub struct TzSelector {
    model: TzSelectorModel,
    widgets: TzSelectorWidgets,
}

impl TzSelector {

    pub fn set_index(&mut self, index: i32) {
        self.model.index = index;
    }

    fn update_time_labels(&self) {
        match &self.model.base_timezone {
            Some(base_zone) => {
                let mut curr_start_time_tz: DateTime<Tz> = Local::now().with_timezone(&Tz::UTC);
                let mut curr_end_time_tz: DateTime<Tz> = Local::now().with_timezone(&Tz::UTC);

                let (opt_curr_start_time_tz, opt_curr_end_time_tz) = get_current_timezone_range(String::from(base_zone), self.model.this_timezone.clone());
                
                if opt_curr_end_time_tz == None || opt_curr_start_time_tz == None {
                    return;
                }

                if let Some(start_tz) = opt_curr_start_time_tz {
                    curr_start_time_tz = start_tz; 
                }

                if let Some(end_tz) = opt_curr_end_time_tz {
                    curr_end_time_tz = end_tz;
                }

                println!("Start time {} / End time {}", curr_start_time_tz, curr_end_time_tz);

                self.widgets.lbl_start.set_text(format!("{}", curr_start_time_tz.format("%I:%M %P")).as_ref());
                self.widgets.lbl_end.set_text(format!("{}", curr_end_time_tz.format("%I:%M %P")).as_ref());

            },
            None => {
            },
        }
    }

    fn update_time_display(&self) {
        let slider_value = self.widgets.slider.get_value();
        let display_value = get_time_string_from_index(slider_value.round(), self.widgets.lbl_start.get_text().as_str());
        self.widgets.lbl_current_select_time.set_text(&display_value);
    }

    fn setup_cmb_liststore(&self) {
        let mut new_cell = CellRendererTextBuilder::new();
        new_cell = new_cell.max_width_chars(25);
        new_cell = new_cell.ellipsize_set(true);
        // new_cell = new_cell.size(20);

        let cell = new_cell.build();
        self.widgets.cmb_tz_name.pack_start(&cell, true);
        self.widgets.cmb_tz_name.add_attribute(&cell, "text", 0);
        self.widgets.cmb_tz_name.set_id_column(0);
        
    }

    fn add_timezone_strings(&self) {
        for tz_name in TZ_VARIANTS.iter() {
            // println!("Item {}", tz_name.name());
            self.add_data_row(&tz_name.name());
        }
    }

    fn add_data_row(&self, col1: &str) {
        let row = self.model.liststore.append();

        self.model.liststore.set_value(&row, 0, &col1.to_value());
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
    
    fn draw_daytime_background(&mut self) {
        let ctx = self.model.draw_handler.get_context();
        let mut curr_start_time_tz: DateTime<Tz> = Local::now().with_timezone(&Tz::UTC);
        let mut base_tz: &str = "";
        // println!("Draw background Base Tz {}, Curr Tz {}", opt_base_tz.clone().unwrap(), opt_this_tz.clone().unwrap());
        
        let (x, y, w, h) = ctx.clip_extents();
    
        if self.model.base_timezone == None {
            return;
        }
    
        if let Some(param_base_tz) = &self.model.base_timezone {
            base_tz = param_base_tz.as_str();
        }
    
        let (opt_curr_start_time_tz, _opt_curr_end_time_tz) = get_current_timezone_range(String::from(base_tz), self.model.this_timezone.clone());
                    
        if opt_curr_start_time_tz == None {
            return;
        }
    
        if let Some(start_tz) = opt_curr_start_time_tz {
            curr_start_time_tz = start_tz;
        }
    
        let offset = calc_offset_for_midday(curr_start_time_tz);
        
        let gr_day = LinearGradient::new(x, y, w, h);
        gr_day.add_color_stop_rgba(offset - 1.0, 0.98, 0.86, 0.12, 0.5);
        gr_day.add_color_stop_rgba(offset - 0.5, 0.2, 0.2, 0.2, 0.5);
        gr_day.add_color_stop_rgba(offset,  0.98, 0.86, 0.12, 0.5);
        gr_day.add_color_stop_rgba(offset + 0.5, 0.2, 0.2, 0.2, 0.5);
        gr_day.add_color_stop_rgba(offset + 1.0,  0.98, 0.86, 0.12, 0.5);
    
        // ctx.set_source_rgba(1.0, 0.2, 0.2, 1.0);
        
        unsafe {
            ctx.set_source(&Pattern::from_raw_none(gr_day.to_raw_none()));
        };
        ctx.paint();
    
        // ctx.set_source_rgba(1.0, 0.2, 0.2, 1.0);
        // ctx.rectangle(x, y, w, h);
        // ctx.fill();
        // ctx.set_source_rgba(0.2, 1.0, 0.2, 1.0);
        // ctx.arc(x/2.0, y/2.0, h/2.0, 0.0, 90.0 * (3.1414 / 180.0));
        ctx.fill();
        
    }
}

// Returns start and end DateTimes for the current timezone based off the start and end times of the base timezones
fn get_current_timezone_range(base_tz: String, this_tz: Option<String>) -> (Option<DateTime<Tz>>, Option<DateTime<Tz>>) {
    
    let (base_start_time_tz, base_end_time_tz) = get_base_timezone_range(base_tz.clone());

    let tz_curr: Tz;
    if let Some(tz) = this_tz.clone() {
        if tz.len() > 0 {
            tz_curr = tz.parse().unwrap();
        } else {
            return (None, None)
        }
    } else {
        //If no timezone is selected for current control do nothing
        return (None, None);
    }

    let curr_start_time_tz = base_start_time_tz.with_timezone(&tz_curr);
    let curr_end_time_tz = base_end_time_tz.with_timezone(&tz_curr);

    return(Some(curr_start_time_tz), Some(curr_end_time_tz));
}

fn get_base_timezone_range(base_tz: String) -> (DateTime<Tz>, DateTime<Tz>) {
    let local_now = Local::now();
    let base_start_time = NaiveDate::from_ymd(local_now.date().year(), local_now.month(), local_now.day()).and_hms(0, 0, 0);
    let base_end_time = NaiveDate::from_ymd(local_now.date().year(), local_now.month(), local_now.day()).and_hms(23, 59, 59);

    let tz_base: Tz = base_tz.parse().unwrap();
    let base_start_time_tz = tz_base.from_local_datetime(&base_start_time).unwrap();
    let base_end_time_tz = tz_base.from_local_datetime(&base_end_time).unwrap();

    return (base_start_time_tz, base_end_time_tz);
}

fn get_time_string_from_index(value: f64, start_time: &str) ->String {
    let starting_time: NaiveTime = NaiveTime::parse_from_str(start_time, "%I:%M %P").unwrap();
    let offset_dur: Duration = Duration::minutes(value as i64 * 15);
    let calc_time = starting_time + offset_dur;
    let ret_string = String::from(format!("{}", calc_time.format("%I:%M %P")));
    return ret_string;
}

fn calc_offset_for_midday(curr_start_time_tz: DateTime<Tz>) -> f64 {
    
    println!("Current Start Time {}", curr_start_time_tz);
    let midday = NaiveTime::from_hms(12, 0, 0);
    let nv_curr = NaiveTime::from_hms(curr_start_time_tz.hour(), curr_start_time_tz.minute(), curr_start_time_tz.second());
    
    let mut offset = (midday - nv_curr).num_minutes();
    if offset < 0 {
        offset = offset + (24 * 60);
    }
    
    let index = ((offset as f64) / 15.0) / 96.0;
    println!("Index {} Minutes Diff {} Hours Diff {}", index, offset, offset / 60);

    return index as f64;
}

impl Update for TzSelector {
    
    type Model = TzSelectorModel;
    type ModelParam = (i32, Option<String>, Option<String>);
    type Msg = Msg;
    
    fn update(&mut self, event: Msg) {
        match event {
            SearchContentsChange => {
                self.model.liststorefilter.refilter();
            },
            SearchKeyReleased(key) => {
                if let Some(key_name) = key.get_keyval().name() {
                    if key_name.as_str() == "Return" {
                        self.widgets.cmb_tz_name.popup();
                    }
                }
            },
            SearchLostFocus => {
                if self.widgets.txt_search_tz.get_text().as_str().len() > 0 {
                    self.widgets.cmb_tz_name.popup();
                }
            },
            DrawIllumination => {
                self.draw_daytime_background();
            },
            RemoveTz => {
                self.model.local_relm.stream().emit(Msg::NotifyParentTzSelectorRemoveClicked(self.model.index));
            },
            NotifyParentTzSelectorRemoveClicked(_) => {
                // Dummy, message is intercepted at win but have to complete match arms here
            },
            LocalTimezoneSelect => {
                let tz_string: String;
                if let Some(sel_str) = self.widgets.cmb_tz_name.get_active_id() {
                    tz_string = String::from(sel_str.as_str());
                } else {
                    return;
                }

                self.model.this_timezone = Some(tz_string.clone());
                self.update_time_labels();
                self.update_time_display();
                //Caught by parent win update loop
                self.model.local_relm.stream().emit(Msg::NotifyParentTimezoneSelectChanged(self.model.index, tz_string.clone()));
                if self.model.index == 0 {
                    self.model.local_relm.stream().emit(Msg::NotifyParentBaseTzChanged(tz_string.clone()));
                }
            },
            LocalTimeSelect(value) => {
                // println!("Value {}", value);
                self.model.local_relm.stream().emit(Msg::NotifyParentTimeSelectChanged(value.round()));
                self.update_time_display();
            },
            NotifyParentTimezoneSelectChanged(_index, _new_zone) => {
                // Dummy, message is intercepted at win but have to complete match arms here
            },
            NotifyParentTimeSelectChanged(_new_value) => {
                // Dummy, message is intercepted at win but have to complete match arms here
            },
            NotifyParentBaseTzChanged(_base_zone) => {
                // Dummy, message is intercepted at win but have to complete match arms here
            },
            FromParentBaseTimezoneChanged(new_zone) => {
                // println!("Base tz change to {}", new_zone);
                self.model.base_timezone = new_zone;
                self.update_time_labels();
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
        let index = param.0;
        let base_timezone = param.1;
        let this_timezone = param.2;
        let liststore = ListStore::new(&[
            Type::String,
        ]);
        
        let liststorefilter = TreeModelFilter::new(&liststore, None); //Probably need a TreePath for a tree not a list like I am using here
        let draw_handler = DrawHandler::new().expect("draw handler");

        TzSelectorModel {
            index,
            base_timezone,
            this_timezone,
            local_relm,
            liststore,
            liststorefilter,
            draw_handler,
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

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
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
        let pb_remove_tz: Button = builder_widget.get_object("pb_remove_tz").expect("Could not get pb_remove_tz");
        let draw_illum: DrawingArea = builder_widget.get_object("draw_illum").expect("Could not get draw_illum");

        connect!(relm, cmb_tz_name, connect_changed(_), Msg::LocalTimezoneSelect);
        connect!(relm, slider, connect_change_value(_, _, val), return (Some(Msg::LocalTimeSelect(val)), Inhibit(false)));
        connect!(relm, txt_search_tz, connect_search_changed(_), Msg::SearchContentsChange);
        connect!(relm, txt_search_tz, connect_key_release_event(_, key), return (Msg::SearchKeyReleased(key.clone()), Inhibit(false)));
        connect!(relm, pb_remove_tz, connect_clicked(_), Msg::RemoveTz);
        connect!(relm, txt_search_tz, connect_focus_out_event(_, _), return(Msg::SearchLostFocus, Inhibit(true)));
        connect!(relm, draw_illum, connect_draw(_, _), return(Msg::DrawIllumination, Inhibit(false)));

        cmb_tz_name.set_model(Some(&model.liststorefilter.clone()));

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
            pb_remove_tz,
            draw_illum,
        };

        TzSelector {
            model,
            widgets,
        }
    }

    fn init_view(&mut self) {
        self.widgets.txt_search_tz.set_property_width_request(20);
        if self.model.index == 0 {
            self.widgets.pb_remove_tz.set_sensitive(false);
            self.widgets.pb_remove_tz.set_visible(false);
        }
        self.setup_cmb_liststore();
        self.add_timezone_strings();
        self.model.draw_handler.init(&self.widgets.draw_illum);

        match self.model.this_timezone.clone() {
            Some(tz_string) => {
                self.widgets.cmb_tz_name.set_active_id(Some(tz_string.clone().as_ref()));
            },
            None => {},
        }

        let style_context = self.widgets.cmb_tz_name.get_style_context();
        // TODO: remove the next line when relm supports css.
        let style = include_bytes!("styling.css");
        let provider = CssProvider::new();
        provider.load_from_data(style).unwrap();
        style_context.add_provider(&provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}
