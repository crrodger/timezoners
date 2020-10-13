use gtk::{Box, Button, ToolButton, Window};
use gtk::{Dialog, Calendar};

#[derive(Clone)]
pub struct MainWidgets {
    pub tz_box: Box,
    pub window: Window,
    pub tb_btn_add_tz: ToolButton,
    pub tb_btn_sel_cal: ToolButton,
    pub dlg_calendar: Dialog,
    pub cal_date: Calendar,
    pub pb_dlg_cal_ok: Button,
    pub pb_dlg_cal_cancel: Button,
}

