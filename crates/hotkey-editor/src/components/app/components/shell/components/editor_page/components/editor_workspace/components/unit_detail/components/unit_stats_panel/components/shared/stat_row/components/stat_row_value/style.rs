use tw_macro::tw;
classes! {
    base: tw![
        "flex-[1_1_auto]",
        "min-w-0",
        "text-right",
        "text-warcraft-text-primary",
        "font-medium",
        "[font-variant-numeric:tabular-nums]",
        "group-data-[variant=hp]:not-data-[zero=true]:text-warcraft-success",
        "group-data-[variant=hp]:font-semibold",
        "group-data-[variant=hp]:text-2xl",
        "group-data-[variant=mana]:not-data-[zero=true]:text-race-human",
        "group-data-[variant=mana]:font-semibold",
        "group-data-[variant=mana]:text-2xl",
        "data-[zero=true]:text-warcraft-text-faint",
        "data-[zero=true]:font-normal",
    ],
    mobile: tw![
        "mobile:group-data-[variant=hp]:text-3xl",
        "mobile:group-data-[variant=mana]:text-3xl",
    ],
}
