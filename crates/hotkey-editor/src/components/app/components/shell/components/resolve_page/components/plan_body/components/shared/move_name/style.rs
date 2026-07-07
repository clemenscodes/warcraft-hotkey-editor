use tw_macro::tw;
classes! {
    base: tw![
        "text-xl",
        "text-warcraft-gold",
        "whitespace-nowrap",
        "min-w-0",
        "data-[link=true]:cursor-pointer",
        "group-[:not(:disabled):hover]:data-[link=true]:text-white",
        "group-[:not(:disabled):hover]:data-[link=true]:underline",
        "group-[:not(:disabled):hover]:data-[link=true]:underline-offset-2",
    ],
    mobile: tw!["mobile:text-base"],
}
