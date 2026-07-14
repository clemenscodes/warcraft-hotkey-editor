use dioxus::prelude::*;

pub struct BurgerMenuView {
    pub(super) is_open: bool,
    pub(super) toggle: EventHandler<MouseEvent>,
    pub(super) on_close: EventHandler<MouseEvent>,
}

pub fn use_burger_menu() -> BurgerMenuView {
    let mut burger_open = use_signal::<bool>(|| false);
    use_effect(move || {
        let is_open = burger_open();
        let Some(body) = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.body())
        else {
            return;
        };
        let style = body.style();
        if is_open {
            let _ = style.set_property("overflow", "hidden");
            let _ = style.set_property("overscroll-behavior", "contain");
        } else {
            let _ = style.remove_property("overflow");
            let _ = style.remove_property("overscroll-behavior");
        }
    });
    let toggle = EventHandler::new(move |_event: MouseEvent| {
        let next = !*burger_open.read();
        burger_open.set(next);
    });
    let on_close = EventHandler::new(move |_event: MouseEvent| burger_open.set(false));
    let is_open = burger_open();
    BurgerMenuView {
        is_open,
        toggle,
        on_close,
    }
}
