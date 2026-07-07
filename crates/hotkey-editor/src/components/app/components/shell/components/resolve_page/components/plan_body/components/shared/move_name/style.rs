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
        "group-[:not(:disabled):hover]:data-[link=true]:[text-underline-offset:2px]",
    ],
    mobile: tw!["mobile:text-[max(0.6rem,min(1.7rem,calc((100vw_-_88px)/27.5)))]"],
}
