use tw_macro::tw;
classes! {
    base: tw![
        "flex-[0_0_auto]",
        "text-warcraft-success",
        "text-xl",
        "font-normal",
        "[font-variant-numeric:tabular-nums]",
        "text-shadow-drop",
        "group-data-[regen=true]:ml-auto",
        "group-data-[regen=true]:text-right",
        "group-data-[regen=true]:group-data-[variant=mana]:not-data-[zero=true]:text-race-human",
        "data-[zero=true]:text-warcraft-text-faint",
        "data-[zero=true]:font-normal",
    ],
    mobile: tw!["mobile:text-xl"],
}
