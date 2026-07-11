use tw_macro::tw;
// The count's dimming follows its breadcrumb's state through the `--count-opacity`
// custom property the active/idle breadcrumb publishes — not a `group-data-[active]`
// selector reaching up to an ancestor's attribute.
classes! {
    base: tw![
        "text-xl",
        "opacity-(--count-opacity,0.8)",
        "before:content-['(']",
        "after:content-[')']",
    ],
}
