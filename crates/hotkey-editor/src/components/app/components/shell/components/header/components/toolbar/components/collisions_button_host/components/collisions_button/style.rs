use tw_macro::tw;
// The collisions button is a thin wrapper around the shared gold `ToolbarButtonSurface`:
// it fills the host box (`size-full`), is the positioning context (`relative`) for the
// absolutely-placed corner count badge, and carries the collision `data-*` attributes and
// aria label the e2e suite reads. All of the button chrome — border, radius, gradient,
// focus ring, hover and resting state — lives on the shared surface it renders, so none of
// it is duplicated here.

classes! {
    base: tw![
        "relative",
        "size-full",
    ],
}
