use tw_macro::tw;
// A layout-neutral identity wrapper: it carries the e2e data attributes and no box
// of its own, so the shared `PageState` shell it wraps becomes the fill item that
// centers the upload prompt.
classes! {
    base: tw!["contents"],
}
