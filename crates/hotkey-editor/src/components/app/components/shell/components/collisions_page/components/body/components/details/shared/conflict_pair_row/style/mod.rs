use tw_macro::tw;

classes! {
    base: tw![
        "grid",
        "grid-cols-[1fr_auto_1fr]",
        "items-start",
        "justify-items-center",
        "gap-[2.88cqi]",
        "w-full",
    ],
    mobile: tw!["mobile:grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]", "mobile:justify-items-stretch"],
    tablet: tw!["tablet:grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]", "tablet:justify-items-stretch"],
}
