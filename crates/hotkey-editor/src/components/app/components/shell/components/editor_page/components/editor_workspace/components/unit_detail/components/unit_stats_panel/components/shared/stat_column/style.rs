use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-2",
        "min-w-0",
        "relative",
        "data-[with-icon=true]:flex-row",
        "data-[with-icon=true]:items-stretch",
        "data-[with-icon=true]:gap-3.5",
        "data-[column=vitality]:[grid-area:vitality]",
        "data-[column=combat]:[grid-area:combat]",
        "data-[column=defense]:[grid-area:defense]",
        "data-[column=attributes]:[grid-area:attributes]",
    ],
}
