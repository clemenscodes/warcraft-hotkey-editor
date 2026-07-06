use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-[0.5rem]",
        "min-w-0",
        "relative",
        "data-[with-icon=true]:flex-row",
        "data-[with-icon=true]:items-stretch",
        "data-[with-icon=true]:gap-[0.9rem]",
        "data-[column=vitality]:[grid-area:vitality]",
        "data-[column=combat]:[grid-area:combat]",
        "data-[column=defense]:[grid-area:defense]",
        "data-[column=attributes]:[grid-area:attributes]",
    ],
}
