use tw_macro::tw;
// The active level option: the shared option chrome values with the gold gradient
// surface and gold text marking it the current level. Shared values with the idle sibling.
classes! {
    base: tw![
        "block","w-full","py-1.5","px-3","border-none","rounded-tile","text-lg","leading-title","text-left","cursor-pointer","whitespace-nowrap","transition-[background,color]","duration-fast","kb-focus:outline-none","kb-focus:text-white","kb-focus:[--focus-color:var(--color-warcraft-highlight)]","kb-focus:shadow-focus",
        "[background:linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-gold)_25%,transparent)_0%,color-mix(in_oklab,var(--color-race-neutral-strong)_70%,transparent)_100%)]",
        "text-warcraft-gold",
    ],
    mobile: tw!["mobile:px-2.5", "mobile:py-1", "mobile:min-h-6", "mobile:text-base"],
    tablet: tw!["tablet:px-2.5", "tablet:py-1", "tablet:min-h-6", "tablet:text-xs"],
    laptop: tw!["laptop:px-2", "laptop:py-1", "laptop:text-base"],
    desktop: tw!["desktop:px-2", "desktop:py-1", "desktop:text-base"],
    qhd: tw!["qhd:px-2", "qhd:py-1", "qhd:text-base"],
    uhd: tw!["uhd:px-2", "uhd:py-1", "uhd:text-base"],
}
