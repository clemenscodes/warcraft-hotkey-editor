use tw_macro::tw;
// A layout-neutral identity wrapper: it carries the e2e data attribute and no box
// of its own, so the shared `PageState` shell it wraps becomes the fill item that
// centers the all-clear message.
classes! {
    base: tw!["contents"],
}
