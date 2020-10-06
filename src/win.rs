use relm::{Relm, Update, Widget, Channel};
use gtk::prelude::*;
use gtk::{Window, Builder, Box, BoxExt,
    MenuItem, Button, ButtonExt, Switch,
    TreeView, TreeViewExt, ListStore, TreeModelFilter, TreeModelFilterExt, TreeViewColumnBuilder, CellRendererTextBuilder, TreeModel, TreeIter,
};
use glib::types::Type;
use crate::relm::ContainerWidget;
use crate::config::*;
use crate::model::*;
use crate::widgets::{MainWidgets, *};
use crate::app::{Msg, MsgUpdateType};

pub struct Win {
    config: Config,
    pub model: Model,
    widgets: MainWidgets,
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
            sender,
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
                // self.save_config();
                gtk::main_quit();
            },
            //Messages from child components
            TimezoneSelectChanged(new_zone) => {
                self.model.tz_ctrls[1].emit(crate::widgets::Msg::BaseTimezoneChanged(format!("{}", new_zone)));
            },
            TimeSelectChanged(new_time) => {
                self.model.tz_ctrls[1].emit(crate::widgets::Msg::BaseTimeSelectChanged(new_time));
            },
            //Messages to child componenets
            ChangeBaseTimezone => {

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
        
        let config: Config = match confy::load("timezoners_gui.glade") {
            Ok(x) =>  x,
            Err(_) => Config::default(),
        };
         
        let glade_src_main = include_str!("timezoners_gui.glade");
        let builder_main = Builder::from_string(glade_src_main);

        //Main window
        let window: Window = builder_main.get_object("main_window").expect("Couldn't get Main Window");
        let menu_item_quit: MenuItem = builder_main.get_object("menu_item_quit").expect("Couldn't get quite menu item");
        // let time_ctrl: Window = builder.get_object("widget_tz_control").expect("Could not get time control window");
        let tz_box: Box = builder_main.get_object("box_widgets").expect("Could not get the widgets box");
        
        let first_selector = tz_box.add_widget::<TzSelector>(());
        connect!(first_selector@crate::widgets::Msg::TimezoneSelectChanged(ref new_zone), relm, Msg::TimezoneSelectChanged(new_zone.clone()));
        connect!(first_selector@crate::widgets::Msg::TimeSelectChanged(new_time), relm, Msg::TimeSelectChanged(new_time));
        let second_selector = tz_box.add_widget::<TzSelector>(());
        
        // connect!(second_selector@crate::widgets::Msg::TimezoneSelectChanged, relm, Msg::TimezoneSelectChanged);
        
        model.tz_ctrls.push(first_selector);
        model.tz_ctrls.push(second_selector);
        
        
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        // connect!(relm, window, connect_show(_), Msg::SetupTree);
        connect!(relm, menu_item_quit, connect_activate(_), Msg::Quit);
        

        window.show_all();
        

        let widgets = MainWidgets {
            tz_box,
            window,
        };

        Win {
            config,
            model,
            widgets,
        }
    }

    fn init_view(&mut self) {
        
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
    

}