use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

// The custom property every gap between the stats rows adds on top of its own
// size, as a plain number of pixels. Unset everywhere else, so the panel keeps
// its plain gaps outside this dialog.
const SPREAD_PROPERTY: &str = "--stats-spread";
// The most a gap grows by. A unit with few stats leaves far more room than its
// rows could fill without falling apart, so beyond this the room stays below
// the panel instead.
const LARGEST_SPREAD_PIXELS: f64 = 10.0;
// The spread the gaps are counted with: the content grows by this much per gap.
const PROBE_SPREAD_PIXELS: f64 = 10.0;

/// Fills the dialog evenly: measures the room the content leaves, counts the
/// gaps between the stats rows, and grows every gap by the same share of that
/// room, up to a limit. It refits whenever the dialog body changes size; the
/// spread only moves the content inside the body, never the body itself, so it
/// never feeds back into its own measurement.
pub(super) struct StatsSpread {
    observer: web_sys::ResizeObserver,
    _on_resize: Closure<dyn FnMut()>,
}

impl StatsSpread {
    pub(super) fn new(body: web_sys::HtmlElement) -> Option<Self> {
        let observed_body = body.clone();
        let on_resize = move || Self::spread(&observed_body);
        let boxed_callback: Box<dyn FnMut()> = Box::new(on_resize);
        let closure = Closure::wrap(boxed_callback);
        let callback = closure.as_ref().unchecked_ref();
        let observer = web_sys::ResizeObserver::new(callback).ok()?;
        observer.observe(&body);
        Some(Self {
            observer,
            _on_resize: closure,
        })
    }

    fn spread(body: &web_sys::HtmlElement) {
        let style = body.style();
        let _ = style.set_property(SPREAD_PROPERTY, "0");
        let Some(compact_height) = Self::content_height(body) else {
            return;
        };
        let probe = format!("{PROBE_SPREAD_PIXELS}");
        let _ = style.set_property(SPREAD_PROPERTY, &probe);
        let Some(probed_height) = Self::content_height(body) else {
            return;
        };
        let gap_count = (probed_height - compact_height) / PROBE_SPREAD_PIXELS;
        let client_height = body.client_height();
        let room = f64::from(client_height);
        let leftover = room - compact_height;
        if gap_count < 1.0 || leftover <= 0.0 {
            let _ = style.set_property(SPREAD_PROPERTY, "0");
            return;
        }
        let even_share = leftover / gap_count;
        let spread_pixels = even_share.min(LARGEST_SPREAD_PIXELS);
        let spread = format!("{spread_pixels:.2}");
        let _ = style.set_property(SPREAD_PROPERTY, &spread);
    }

    /// How tall the content is, from the top of the body to the bottom of its
    /// last child plus the body's bottom padding. Unlike `scrollHeight` this is
    /// the content's own height even when it is shorter than the body.
    fn content_height(body: &web_sys::HtmlElement) -> Option<f64> {
        let last_child = body.last_element_child()?;
        let last_bottom = last_child.get_bounding_client_rect().bottom();
        let body_top = body.get_bounding_client_rect().top();
        let window = web_sys::window()?;
        let computed_style = window.get_computed_style(body).ok()??;
        let padding_bottom = computed_style.get_property_value("padding-bottom").ok()?;
        let padding_pixels = padding_bottom.trim_end_matches("px").parse::<f64>().ok()?;
        Some(last_bottom - body_top + padding_pixels)
    }
}

impl Drop for StatsSpread {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}
