use tw_macro::tw;
classes! {
    base: tw![
        "text-lg",
        "text-warcraft-gold",
        "text-shadow-drop",
    ],
    // The move count is a subtitle under the plan name, so on a phone it steps
    // below the title size rather than sitting larger than it.
    mobile: tw![
        "mobile:text-xs",
    ],
}
