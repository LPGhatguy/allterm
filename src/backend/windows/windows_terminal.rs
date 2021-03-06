use crate::{
    style::Style,
    key::Key,
    terminal_backend::TerminalBackend,
    os::windows::{enable_raw_mode, disable_raw_mode, get_terminal_size},
};

pub struct WindowsTerminal;

impl TerminalBackend for WindowsTerminal {
    fn enable_raw_mode(&mut self) -> Result<(), String> {
        enable_raw_mode()
    }

    fn disable_raw_mode(&mut self) -> Result<(), String> {
        disable_raw_mode()
    }

    fn enable_alternate_screen(&mut self) {
        unimplemented!()
    }

    fn disable_alternate_screen(&mut self) {
        unimplemented!()
    }

    fn clear_screen(&mut self) {
        unimplemented!()
    }

    fn hide_cursor(&mut self) {
        unimplemented!()
    }

    fn show_cursor(&mut self) {
        unimplemented!()
    }

    fn move_cursor(&mut self, _x: usize, _y: usize) {
        unimplemented!()
    }

    fn print(&mut self, _text: &str, _style: Style) {
        unimplemented!()
    }

    fn get_size(&self) -> Result<(usize, usize), String> {
        get_terminal_size()
    }

    fn read_key(&mut self) -> Key {
        unimplemented!()
    }
}