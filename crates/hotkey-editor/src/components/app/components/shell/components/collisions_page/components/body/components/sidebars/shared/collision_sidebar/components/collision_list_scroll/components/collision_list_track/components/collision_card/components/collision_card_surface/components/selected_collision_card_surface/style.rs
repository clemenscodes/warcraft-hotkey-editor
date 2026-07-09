use tw_macro::tw;
// The selected collision card surface: shared chrome values on the blue surface, wearing
// the fixed collision-gold accent (border, text, glow) and publishing
// `--coordinate-color: gold` for its coordinate. Shared values with the idle sibling.
classes! {
    base: tw![
        "flex","items-center","gap-4","p-4","w-full","min-w-0","text-left","text-lg","tracking-snug","border","rounded-tile","transition-all","duration-fast","bg-warcraft-bg-mid/55","text-warcraft-text-primary","kb-focus:border-white","kb-focus:text-white","kb-focus:bg-warcraft-blue/85","kb-focus:shadow-focus",
        "bg-panel-blue",
        "border-warcraft-gold",
        "text-warcraft-gold",
        "shadow-glow-soft",
        "[--coordinate-color:var(--color-warcraft-gold)]",
    ],
    mobile: tw!["mobile:h-full","mobile:py-2","mobile:px-2.5","mobile:gap-2.5","mobile:box-border","mobile:overflow-hidden","mobile:bg-panel-dark","mobile:border-warcraft-blue/60"],
    tablet: tw!["tablet:h-full","tablet:py-2","tablet:px-2.5","tablet:gap-2.5","tablet:box-border","tablet:overflow-hidden","tablet:bg-panel-dark","tablet:border-warcraft-blue/60"],
}
