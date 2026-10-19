use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Fits the title on one line: it renders at its token size, and when that is
/// wider than the room between the flourishes it shrinks its font by exactly
/// the share it overflows. Text width scales with font size, so one measurement
/// lands it flush. It refits whenever the header changes width, and only then,
/// so the font it sets never feeds back into its own measurement.
pub(super) struct TitleFit {
    observer: web_sys::ResizeObserver,
    _on_resize: Closure<dyn FnMut()>,
}

impl TitleFit {
    pub(super) fn new(title: web_sys::HtmlElement) -> Option<Self> {
        let header = title.parent_element()?;
        let observed_header = header.clone();
        let mut fitted_header_width: i32 = -1;
        let on_resize = move || {
            let header_width = observed_header.client_width();
            if header_width == fitted_header_width {
                return;
            }
            fitted_header_width = header_width;
            Self::fit(&title);
        };
        let boxed_callback: Box<dyn FnMut()> = Box::new(on_resize);
        let closure = Closure::wrap(boxed_callback);
        let callback = closure.as_ref().unchecked_ref();
        let observer = web_sys::ResizeObserver::new(callback).ok()?;
        observer.observe(&header);
        Some(Self {
            observer,
            _on_resize: closure,
        })
    }

    fn fit(title: &web_sys::HtmlElement) {
        let style = title.style();
        let _ = style.remove_property("font-size");
        let room = title.client_width();
        let needed = title.scroll_width();
        if needed <= room || room <= 0 {
            return;
        }
        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(Some(computed_style)) = window.get_computed_style(title) else {
            return;
        };
        let Ok(token_size) = computed_style.get_property_value("font-size") else {
            return;
        };
        let Ok(token_pixels) = token_size.trim_end_matches("px").parse::<f64>() else {
            return;
        };
        let room_pixels = f64::from(room);
        let needed_pixels = f64::from(needed);
        let fitted_pixels = token_pixels * room_pixels / needed_pixels;
        let fitted_size = format!("{fitted_pixels:.2}px");
        let _ = style.set_property("font-size", &fitted_size);
    }
}

impl Drop for TitleFit {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}
