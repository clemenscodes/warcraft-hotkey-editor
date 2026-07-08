use tw_macro::tw;
// The collisions page's outer box for the shared mini grid frame: a fixed width per
// band (which sets the frame's scale) that never shrinks, rounded at the hairline
// radius. `overflow-hidden` clips the frame's panel surface and border to that
// radius.

classes! {
    base: tw![
        "shrink-0",
        "w-[106.67px]",
        "rounded-hairline",
        "overflow-hidden",
    ],
    mobile: tw!["mobile:w-[88px]"],
    tablet: tw!["tablet:w-[122.67px]"],
}
