use zellij_tile::{
    prelude::{Context, Event, LoggableError},
    register_plugin,
    shim::{detach, report_panic, get_plugin_ids, close_plugin_pane},
    ZellijPlugin,
};

#[derive(Default)]
struct Detach {}

register_plugin!(Detach);

impl ZellijPlugin for Detach {
    fn load(&mut self) {
        detach();
        close_plugin_pane(get_plugin_ids().plugin_id as i32);
        std::process::exit(0);
    }

    fn update(&mut self, _event: Event) -> bool {
        false
    }

    fn render(&mut self, _rows: usize, _cols: usize) {}
}
