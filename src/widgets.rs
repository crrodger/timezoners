use gtk::{Box, Button, ToolButton, Window, ColorChooser,};
use gtk::{Dialog, Calendar};

#[derive(Clone)]
pub struct MainWidgets {
    pub tz_box: Box,
    pub window: Window,
    pub tb_btn_add_tz: ToolButton,
    pub tb_btn_sel_cal: ToolButton,
    pub tb_btn_sel_col: ToolButton,
    pub dlg_calendar: Dialog,
    pub cal_date: Calendar,
    pub pb_dlg_cal_ok: Button,
    pub pb_dlg_cal_cancel: Button,
    pub dlg_colour: Dialog,
    pub dlg_col_col_midday: ColorChooser, 
    pub dlg_col_col_workday: ColorChooser,
    pub pb_dlg_col_ok: Button, 
    pub pb_dlg_col_cancel: Button,
}

