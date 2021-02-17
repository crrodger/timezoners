use relm::{Relm, Update, Widget, Channel, WidgetTest};
use gtk::prelude::*;
use gtk::{Window, Builder, Box, 
    ToolButton, Dialog, Button, Calendar, STYLE_PROVIDER_PRIORITY_APPLICATION, CssProvider,
    ColorChooser,
};
use gdk::{RGBA};
use chrono::{NaiveDate, Local, Datelike};
use crate::relm::ContainerWidget;
use crate::model::*;
use crate::widgets::MainWidgets;
use crate::tzselector::*;
use crate::app::{Msg, MsgUpdateType};

pub struct Win {
    pub model: Model,
    widgets: MainWidgets,
    config: Config,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        
        let stream = relm.stream().clone();
        let (_channel, sender) = Channel::new(move |upd_tuple| {
            // This closure is executed whenever a message is received from the sender.
            // We send a message to the current widget.
            stream.emit(Msg::ProcessUpdateMsg(upd_tuple));
        });
        let local_now = Local::now();
        let for_date = NaiveDate::from_ymd(local_now.year(), local_now.month(), local_now.day());

        Model {
            tz_ctrls: vec![],
            tz_zones: vec![],
            sender,
            local_relm: relm.clone(),
            base_tz: None,
            for_date,
        }
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;
        match event {
            // Intended to provide an example of using async messages - more for use in multi-threaded use
            ProcessUpdateMsg((msg_type , msg_str)) => {
                match msg_type {
                    MsgUpdateType::StatusMessage => {
                        println!("StatusMessage -> {}", msg_str);
                    }
                }
            },
            Quit => {
                self.config.zones.clear();
                for i in 0..self.model.tz_zones.len() {
                    if let Some(tz_zone) = self.model.tz_zones[i].clone() {
                        self.config.zones.push(Some(tz_zone));
                    }
                    
                }
                self.save_config();
                gtk::main_quit();
            },
            AddTzSelector(tz_location) => {
                self.add_tz_selector(tz_location);
            },
            SelectDate => {
                self.widgets.cal_date.select_month(self.model.for_date.month()-1, self.model.for_date.year() as u32);
                self.widgets.cal_date.select_day(self.model.for_date.day());
                self.widgets.dlg_calendar.show_all();
            },
            DateOkay => {
                let (y,m,d) = self.widgets.cal_date.get_date();
                self.model.for_date = NaiveDate::from_ymd(y as i32, m + 1, d);
                self.widgets.dlg_calendar.hide();
                self.widgets.tb_btn_sel_cal.set_label(Some(format!("{}", self.model.for_date.format("On %Y/%m/%d")).as_ref()));
                for i in 1..self.model.tz_ctrls.len() {
                    self.model.tz_ctrls[i].emit(crate::tzselector::Msg::FromParentDateChanged(self.model.for_date));
                };
            },
            DateCancel => {
                self.widgets.dlg_calendar.hide();
            },
            SelectColour => {
                self.widgets.dlg_col_col_midday.set_rgba(&RGBA {
                                                red:     self.config.midday_colour.0, 
                                                green:   self.config.midday_colour.1,
                                                blue:    self.config.midday_colour.2,
                                                alpha:   self.config.midday_colour.3});
                self.widgets.dlg_col_col_workday.set_rgba(&RGBA {
                                                    red:     self.config.workday_colour.0, 
                                                    green:   self.config.workday_colour.1,
                                                    blue:    self.config.workday_colour.2,
                                                    alpha:   self.config.workday_colour.3});
                self.widgets.dlg_colour.show_all();
            },
            ColourOkay => {
                let midday_colour = self.widgets.dlg_col_col_midday.get_rgba();
                let workday_colour = self.widgets.dlg_col_col_workday.get_rgba();
                self.config.midday_colour = (midday_colour.red, midday_colour.green, midday_colour.blue, midday_colour.alpha);
                self.config.workday_colour = (workday_colour.red, workday_colour.green, workday_colour.blue, workday_colour.alpha);
                self.widgets.dlg_colour.hide();
                for i in 0..self.model.tz_ctrls.len() {
                    self.model.tz_ctrls[i].emit(crate::tzselector::Msg::FromParentColourChanged(self.config.midday_colour, self.config.workday_colour));
                }
            },
            ColourCancel => {
                self.widgets.dlg_colour.hide();
            },
            //Messages from child components
            TimezoneSelectChanged(index, new_zone) => {
                self.model.tz_zones[index as usize] = Some(new_zone);
            },
            TimeSelectChanged(new_time) => {
                for i in 0..self.model.tz_ctrls.len() {
                    self.model.tz_ctrls[i].emit(crate::tzselector::Msg::FromParentBaseTimeSelectChanged(new_time));
                }
            },
            TimezoneRemove(remove_index) => {
                let rem_widget = self.model.tz_ctrls.get(remove_index as usize).unwrap();
                self.widgets.tz_box.remove::<Box>(rem_widget.widget());
                self.model.tz_zones[remove_index as usize] = None;
            },
            //Messages to child componenets
            ChangeBaseTimezone(new_zone) => {
                self.model.base_tz = new_zone.clone();
                for i in 0..self.model.tz_ctrls.len() {
                    self.model.tz_ctrls[i].emit(crate::tzselector::Msg::FromParentBaseTimezoneChanged(new_zone.clone()));
                }
            },
            SetToNow => {
                self.model.tz_ctrls[0].emit(crate::tzselector::Msg::FromParentSetToNow);
            },
        }
    }
    
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, mut model: Self::Model) -> Self {
        let mut base_tz: Option<String> = None;

        let config: Config = match confy::load("TimezoneRS") {
            Ok(x) =>  x,
            Err(_) => Config::default(),
        };

        if config.zones.len() > 0 {
            if let Some(tz_string) = config.zones[0].clone() {
                base_tz = Some(tz_string.clone());
            }
            
        }
        model.base_tz = base_tz.clone();

        let glade_src_main = include_str!("timezoners_gui.glade");
        let builder_main = Builder::from_string(glade_src_main);

        //Main window
        let window: Window = builder_main.get_object("main_window").expect("Couldn't get Main Window");
        let tz_box: Box = builder_main.get_object("box_widgets").expect("Could not get the widgets box");
        let tb_btn_sel_exit: ToolButton = builder_main.get_object("tb_btn_sel_exit").expect("Couldn't get exit button tb_btn_sel_exit");
        let tb_btn_add_tz: ToolButton = builder_main.get_object("tb_btn_add_tz").expect("Could not get tb_btn_add_tz");
        let tb_btn_sel_cal: ToolButton = builder_main.get_object("tb_btn_sel_cal").expect("Could not geto tb_btn_sel_cal");
        let tb_btn_sel_col: ToolButton = builder_main.get_object("tb_btn_sel_col").expect("Could not get tool button tb_btn_sel_col");
        let tb_btn_sel_now: ToolButton = builder_main.get_object("tb_btn_sel_now").expect("Could not get tool button tb_btn_sel_now");
        
        let dlg_calendar: Dialog = builder_main.get_object("dlg_calendar").expect("Could not get dialog dlg_calendar");
        let cal_date: Calendar = builder_main.get_object("cal_date").expect("Could not get cal_date");
        let pb_dlg_cal_ok: Button = builder_main.get_object("pb_dlg_cal_ok").expect("Could not get button pb_dlg_cal_ok");
        let pb_dlg_cal_cancel: Button = builder_main.get_object("pb_dlg_cal_cancel").expect("Could not get button pb_dlg_cal_cancel");
        
        let dlg_colour: Dialog  = builder_main.get_object("dlg_colour").expect("Could not get dialog dlg_colour");
        let dlg_col_col_midday: ColorChooser = builder_main.get_object("dlg_col_col_midday").expect("Could not get colour chooser dlg_col_col_midday");
        let dlg_col_col_workday: ColorChooser = builder_main.get_object("dlg_col_col_workday").expect("Could not get colour chooser dlg_col_col_workday");
        let pb_dlg_col_ok: Button = builder_main.get_object("pb_dlg_col_ok").expect("Could not get button pb_dlg_col_ok");
        let pb_dlg_col_cancel: Button = builder_main.get_object("pb_dlg_col_cancel").expect("Could not get button pb_dlg_col_cancel");


        let midday_colour = (
            config.midday_colour.0, 
            config.midday_colour.1,
            config.midday_colour.2,
            config.midday_colour.3);

        let workday_colour = (
            config.workday_colour.0, 
            config.workday_colour.1,
            config.workday_colour.2,
            config.workday_colour.3);
        
        let first_selector = tz_box.add_widget::<TzSelector>((0, base_tz.clone(), base_tz.clone(), model.for_date.clone(), midday_colour, workday_colour));
        
        connect!(first_selector@crate::tzselector::Msg::NotifyParentTimezoneSelectChanged(index, ref new_zone), relm, Msg::TimezoneSelectChanged(index, new_zone.clone()));
        connect!(first_selector@crate::tzselector::Msg::NotifyParentTimeSelectChanged(new_time), relm, Msg::TimeSelectChanged(new_time));
        connect!(first_selector@crate::tzselector::Msg::NotifyParentBaseTzChanged(ref new_zone), relm, Msg::ChangeBaseTimezone(Some(new_zone.clone())));
        
        
        model.tz_ctrls.push(first_selector);
        model.tz_zones.push(base_tz);
        
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        connect!(relm, tb_btn_sel_exit, connect_clicked(_), Msg::Quit);
        connect!(relm, tb_btn_add_tz, connect_clicked(_), Msg::AddTzSelector(String::from("")));
        connect!(relm, tb_btn_sel_cal, connect_clicked(_), Msg::SelectDate);
        connect!(relm, pb_dlg_cal_ok, connect_clicked(_), Msg::DateOkay);
        connect!(relm, pb_dlg_cal_cancel, connect_clicked(_), Msg::DateCancel);

        connect!(relm, tb_btn_sel_col, connect_clicked(_), Msg::SelectColour);
        connect!(relm, tb_btn_sel_now, connect_clicked(_), Msg::SetToNow);
        connect!(relm, pb_dlg_col_ok, connect_clicked(_), Msg::ColourOkay);
        connect!(relm, pb_dlg_col_cancel, connect_clicked(_), Msg::ColourCancel);
        
        window.show_all();
        window.move_(config.win_pos_x, config.win_pos_y);
        window.resize(config.win_width, config.win_height);
        window.present();
        
        let widgets = MainWidgets {
            tz_box,
            window,
            tb_btn_add_tz,
            tb_btn_sel_cal,
            tb_btn_sel_col,
            tb_btn_sel_now,
            dlg_calendar,
            cal_date,
            pb_dlg_cal_ok,
            pb_dlg_cal_cancel,
            dlg_colour,
            dlg_col_col_midday,
            dlg_col_col_workday,
            pb_dlg_col_ok,
            pb_dlg_col_cancel, 
        };

        Win {
            model,
            widgets,
            config,
        }
    }

    fn init_view(&mut self) {
        if self.config.zones.len() > 0 {
            for i in 1..self.config.zones.len() {
                if let Some(tz_location) = self.config.zones[i].clone() {
                    if tz_location.len() > 0 {
                        self.add_tz_selector(tz_location);
                    }
                }
            }
            self.model.local_relm.stream().emit(Msg::SetToNow);
        }

        self.widgets.tb_btn_sel_cal.set_label(Some(format!("{}", self.model.for_date.format("On %Y/%m/%d")).as_ref()));

        let style_context = self.widgets.tz_box.get_style_context();
        let style = include_bytes!("styling.css");
        let provider = CssProvider::new();
        provider.load_from_data(style).unwrap();
        style_context.add_provider(&provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

        self.widgets.tz_box.set_border_width(3);
        

    }

    fn run(model_param: Self::ModelParam) -> Result<(), ()>
        where Self: 'static,
    {
        relm::run::<Self>(model_param)
    }
}

impl WidgetTest for Win {
    type Streams = ();

    fn get_streams(&self) -> Self::Streams {
    }

    type Widgets = MainWidgets;

    fn get_widgets(&self) -> Self::Widgets {
        self.widgets.clone()
    }
}



impl Win {
    fn add_tz_selector(&mut self, tz_location: String) {
        let midday_colour = (
            self.config.midday_colour.0, 
            self.config.midday_colour.1,
            self.config.midday_colour.2,
            self.config.midday_colour.3
        );

        let workday_colour = (
            self.config.workday_colour.0, 
            self.config.workday_colour.1,
            self.config.workday_colour.2,
            self.config.workday_colour.3
        );

        let new_selector = self.widgets.tz_box.add_widget::<TzSelector>((self.model.tz_ctrls.len() as i32, self.model.base_tz.clone(), Some(tz_location.clone()), self.model.for_date.clone(), midday_colour, workday_colour));
        connect!(new_selector@crate::tzselector::Msg::NotifyParentTimeSelectChanged(new_time), self.model.local_relm, Msg::TimeSelectChanged(new_time));
        connect!(new_selector@crate::tzselector::Msg::NotifyParentTzSelectorRemoveClicked(remove_index), self.model.local_relm, Msg::TimezoneRemove(remove_index));
        connect!(new_selector@crate::tzselector::Msg::NotifyParentTimezoneSelectChanged(index, ref new_zone), self.model.local_relm, Msg::TimezoneSelectChanged(index, new_zone.clone()));
        
        self.model.tz_ctrls.push(new_selector);
        self.model.tz_zones.push(Some(tz_location));
        
    }

    fn save_config(&mut self) {
        let (x,y) = self.widgets.window.get_position();
        let (w,h) = self.widgets.window.get_size();
        self.config.win_pos_x = x;
        self.config.win_pos_y = y;
        self.config.win_width = w;
        self.config.win_height = h;

        match confy::store("TimezoneRS", &self.config) {
            Ok(_) => {},
            Err(_) => {},
        }
    }

}

#[cfg(test)]
mod tests {
    use gtk::{ToolButtonExt, ContainerExt, Box, Entry, Label, LabelExt, EntryExt};
    use gtk_test::{assert_label, assert_text};
    use relm_test::{Observer, click, relm_observer_new, relm_observer_wait, enter_key, key_press, key_release};
    use relm::{Cast};
    use gdk::keys::constants as key;

    use crate::win::Win;

    #[test]
    fn main_window_created() {
        let (_component, _, widgets) = relm::init_test::<Win>(()).expect("init_test failed");
        let tb_btn_add_tz = &widgets.tb_btn_add_tz;
        let tb_btn_sel_cal = &widgets.tb_btn_sel_cal;
        let tb_btn_sel_col = &widgets.tb_btn_sel_col;
        
        let today = chrono::Local::now();
        let today_string = today.format("On %G/%m/%d");
        
        assert_label!(tb_btn_add_tz, "Add");
        assert_label!(tb_btn_sel_cal, today_string);
        assert_label!(tb_btn_sel_col, "Colour");

    }

    #[test]
    fn create_tz_selector() {
        let (component, _, widgets) = relm::init_test::<Win>(()).expect("init_test failed");

        let tb_btn_add_tz = &widgets.tb_btn_add_tz;

        let observer = relm_observer_new!(component, AddTzSelector);
        
        click(tb_btn_add_tz);
        relm_observer_wait!(let AddTzSelector = observer);

        observer.wait();

    }

    #[test]
    fn enter_times() {
        let (component, _,  widgets) = relm::init_test::<Win>(()).expect("init_test failed");

        let tz_selector_main = component.widget();
        let window_box = tz_selector_main.get_children();
        
        let main_box = window_box.get(0).unwrap().clone().downcast::<Box>().expect("Could not get the main box");
        let kids = main_box.get_children();
        let widgets_box = kids.get(1).unwrap().clone().downcast::<Box>().expect("Could not get widgets box");
        let kids = widgets_box.get_children();
        let tz_box = kids.get(0).unwrap().clone().downcast::<Box>().expect("Could not get first tz selector box");
        let kids = tz_box.get_children();
        let left_box = kids.get(0).unwrap().clone().downcast::<Box>().expect("Could not get tz selector left box");
        let right_box = kids.get(1).unwrap().clone().downcast::<Box>().expect("Could not get tz selector right box");
        
        let kids = right_box.get_children();
        let centre_box = kids.get(0).unwrap().clone().downcast::<Box>().expect("Could not get centre box");
        let kids = centre_box.get_children();
        let labels_box = kids.get(0).unwrap().clone().downcast::<Box>().expect("Could not get labels box");
        let kids = labels_box.get_children();
        // eprintln!("{:#?}", kids);
        let time_label = kids.get(1).unwrap().clone().downcast::<Label>().expect("Could not get current time label");
        
        assert_text!(time_label, "12:00 pm");
        let kids = left_box.get_children();
        let time_entry = kids.get(1).unwrap().clone().downcast::<Entry>().expect("Could not get time entry");
        
        enter_key(&time_entry, key::KP_1);
        assert_text!(time_entry, "1");
        enter_key(&time_entry, key::KP_1);
        assert_text!(time_entry, "11");
        key_press(&time_entry, key::Shift_L);
        key_press(&time_entry, key::colon);
        key_release(&time_entry, key::Shift_L);
        assert_text!(time_entry, "11:");
        enter_key(&time_entry, key::KP_0);
        assert_text!(time_entry, "11:0");
        enter_key(&time_entry, key::KP_0);
        
        assert_text!(time_label, "11:00 am");

        //Check a time in between 15 minute range
        enter_key(&time_entry, key::KP_1);
        assert_text!(time_entry, "1");
        enter_key(&time_entry, key::KP_1);
        assert_text!(time_entry, "11");
        key_press(&time_entry, key::Shift_L);
        key_press(&time_entry, key::colon);
        key_release(&time_entry, key::Shift_L);
        assert_text!(time_entry, "11:");
        enter_key(&time_entry, key::KP_0);
        assert_text!(time_entry, "11:0");
        enter_key(&time_entry, key::KP_6);
        
        assert_text!(time_label, "11:00 am");

        //Check a time in between 15 minute range
        enter_key(&time_entry, key::KP_1);
        assert_text!(time_entry, "1");
        enter_key(&time_entry, key::KP_1);
        assert_text!(time_entry, "11");
        key_press(&time_entry, key::Shift_L);
        key_press(&time_entry, key::colon);
        key_release(&time_entry, key::Shift_L);
        assert_text!(time_entry, "11:");
        enter_key(&time_entry, key::KP_0);
        assert_text!(time_entry, "11:0");
        enter_key(&time_entry, key::KP_9);
        
        assert_text!(time_label, "11:15 am");

        //Check an invalid time string
        enter_key(&time_entry, key::A);
        assert_text!(time_entry, "a");
        enter_key(&time_entry, key::KP_1);
        assert_text!(time_entry, "a1");
        key_press(&time_entry, key::L);
        assert_text!(time_entry, "a1l");
        enter_key(&time_entry, key::space);
        assert_text!(time_entry, "a1l ");
        enter_key(&time_entry, key::KP_9);
        
        assert_text!(time_label, "12:00 am");
        
    }
}