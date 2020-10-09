use relm::{Relm, Update, Widget, Channel};
use gtk::prelude::*;
use gtk::{Window, Builder, Box, 
    MenuItem, ToolButton,
};
use crate::relm::ContainerWidget;
use crate::model::*;
use crate::widgets::{MainWidgets, *};
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

        Model {
            tz_ctrls: vec![],
            tz_zones: vec![],
            sender,
            local_relm: relm.clone(),
            base_tz: None,
        }
    }

    fn update(&mut self, event: Msg) {
        use Msg::*;
        match event {
            // Intended to provide a demo of using async messages - more for use in multi-threaded use
            ProcessUpdateMsg((msg_type , msg_str)) => {
                match msg_type {
                    MsgUpdateType::StatusMessage => {
                        println!("StatsMessage -> {}", msg_str);
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
            //Messages from child components
            TimezoneSelectChanged(index, new_zone) => {
                self.model.tz_zones[index as usize] = Some(new_zone);
                // for i in 0..self.model.tz_ctrls.len() {
                //     self.model.tz_ctrls[i].emit(crate::widgets::Msg::FromParentBaseTimezoneChanged(format!("{}", new_zone)));
                // }
            },
            TimeSelectChanged(new_time) => {
                for i in 1..self.model.tz_ctrls.len() {
                    self.model.tz_ctrls[i].emit(crate::widgets::Msg::FromParentBaseTimeSelectChanged(new_time));
                }
            },
            TimezoneRemove(remove_index) => {
                let rem_widget = self.model.tz_ctrls.get(remove_index as usize).unwrap();
                // println!("{:#?}", rem_widget);
                self.widgets.tz_box.remove::<Box>(rem_widget.widget());
                self.model.tz_zones[remove_index as usize] = None;
            },
            //Messages to child componenets
            ChangeBaseTimezone(new_zone) => {
                self.model.base_tz = new_zone.clone();
                for i in 1..self.model.tz_ctrls.len() {
                    self.model.tz_ctrls[i].emit(crate::widgets::Msg::FromParentBaseTimezoneChanged(new_zone.clone()));
                }
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
        let menu_item_quit: MenuItem = builder_main.get_object("menu_item_quit").expect("Couldn't get quite menu item");
        // let time_ctrl: Window = builder.get_object("widget_tz_control").expect("Could not get time control window");
        let tz_box: Box = builder_main.get_object("box_widgets").expect("Could not get the widgets box");
        let tb_btn_add_tz: ToolButton = builder_main.get_object("tb_btn_add_tz").expect("Could not get tb_btn_add_tz");
        
        let first_selector = tz_box.add_widget::<TzSelector>((0, base_tz.clone(), base_tz.clone()));
        // first_selector.set_index(0);
        connect!(first_selector@crate::widgets::Msg::NotifyParentTimezoneSelectChanged(index, ref new_zone), relm, Msg::TimezoneSelectChanged(index, new_zone.clone()));
        connect!(first_selector@crate::widgets::Msg::NotifyParentTimeSelectChanged(new_time), relm, Msg::TimeSelectChanged(new_time));
        connect!(first_selector@crate::widgets::Msg::NotifyParentBaseTzChanged(ref new_zone), relm, Msg::ChangeBaseTimezone(Some(new_zone.clone())));
        
        // connect!(second_selector@crate::widgets::Msg::TimezoneSelectChanged, relm, Msg::TimezoneSelectChanged);
        
        model.tz_ctrls.push(first_selector);
        model.tz_zones.push(base_tz);
        
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        // connect!(relm, window, connect_show(_), Msg::SetupTree);
        connect!(relm, menu_item_quit, connect_activate(_), Msg::Quit);
        connect!(relm, tb_btn_add_tz, connect_clicked(_), Msg::AddTzSelector(String::from("")));
        
        window.show_all();
        window.move_(config.win_pos_x, config.win_pos_y);
        window.resize(config.win_width, config.win_height);
        
        let widgets = MainWidgets {
            tz_box,
            window,
            tb_btn_add_tz,
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
        }

        

    }

    fn on_add<W: IsA<gtk::Widget> + IsA<glib::Object>>(&self, _parent: W) {

    }

    fn parent_id() -> Option<&'static str> {
        None
    }
    fn run(model_param: Self::ModelParam) -> Result<(), ()>
        where Self: 'static,
    {
        relm::run::<Self>(model_param)
    }
}


impl Win {
    fn add_tz_selector(&mut self, tz_location: String) {
        let new_selector = self.widgets.tz_box.add_widget::<TzSelector>((self.model.tz_ctrls.len() as i32, self.model.base_tz.clone(), Some(tz_location.clone())));
        connect!(new_selector@crate::widgets::Msg::NotifyParentTzSelectorRemoveClicked(remove_index), self.model.local_relm, Msg::TimezoneRemove(remove_index));
        connect!(new_selector@crate::widgets::Msg::NotifyParentTimezoneSelectChanged(index, ref new_zone), self.model.local_relm, Msg::TimezoneSelectChanged(index, new_zone.clone()));
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