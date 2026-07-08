use tw_macro::tw;
// The resolve page's outer box for the shared mini grid frame: it grows to fill the
// move row, spans the full width (which sets the frame's scale), and rounds the
// corners at the control radius. `overflow-hidden` clips the frame's panel surface
// and border to that radius.

classes! {
    base: tw![
        "flex-[1_1_auto]",
        "w-full",
        "min-w-0",
        "rounded-control",
        "overflow-hidden",
    ],
}
