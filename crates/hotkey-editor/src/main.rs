use hotkey_editor::components::app::App;

fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}
