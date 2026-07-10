use tw_macro::tw;
// The idle level option: the shared option chrome values, transparent, lighting gold on
// hover. Shared values with the active sibling.
classes! {
    base: tw![
        "block","w-full","py-1.5","px-3","border-none","rounded-tile","text-lg","leading-title","text-left","cursor-pointer","whitespace-nowrap","transition-[background,color]","duration-fast","kb-focus:outline-none","kb-focus:text-white","kb-focus:[--focus-color:var(--color-warcraft-highlight)]","kb-focus:shadow-focus",
        "bg-transparent",
        "text-warcraft-text-secondary",
        "hover:bg-warcraft-gold/12",
        "hover:text-warcraft-gold",
    ],
    mobile: tw!["mobile:px-2.5", "mobile:py-1", "mobile:min-h-6", "mobile:text-base"],
    tablet: tw!["tablet:px-2.5", "tablet:py-1", "tablet:min-h-6", "tablet:text-xs"],
    laptop: tw!["laptop:px-2", "laptop:py-1", "laptop:text-base"],
    desktop: tw!["desktop:px-2", "desktop:py-1", "desktop:text-base"],
    qhd: tw!["qhd:px-2", "qhd:py-1", "qhd:text-base"],
    uhd: tw!["uhd:px-2", "uhd:py-1", "uhd:text-base"],
}
