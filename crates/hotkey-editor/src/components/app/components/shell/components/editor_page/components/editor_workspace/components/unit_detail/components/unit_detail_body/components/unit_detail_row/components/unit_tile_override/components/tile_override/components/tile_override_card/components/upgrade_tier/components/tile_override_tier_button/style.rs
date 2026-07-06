use tw_macro::tw;
// A prev/next arrow button in the tier-cycling footer. Small bronze square that
// golds on hover; the injected arrow SVG is centered and fixed-size. Class
// `.tile-override-tier-button` is load-bearing (keyboard navigation).

classes! {
    base: tw![
        "w-[2.4rem]",
        "h-[2.4rem]",
        "p-0",
        "flex",
        "items-center",
        "justify-center",
        "bg-warcraft-gold-dark/55",
        "border",
        "border-warcraft-gold-border",
        "rounded-[0.25rem]",
        "cursor-pointer",
        "transition-[border-color,background]",
        "duration-[0.12s]",
        "hover:border-warcraft-gold",
        "hover:bg-warcraft-gold/12",
        "[&>span]:block",
        "[&_svg]:w-[1.7rem]",
        "[&_svg]:h-[1.7rem]",
    ],
    mobile: tw![
        "mobile:w-[34px]",
        "mobile:h-[34px]",
        "mobile:min-w-[34px]",
        "mobile:min-h-[34px]",
        "mobile:[&_svg]:w-[22px]",
        "mobile:[&_svg]:h-[22px]",
    ],
    tablet: tw![
        "tablet:w-[34px]",
        "tablet:h-[34px]",
        "tablet:min-w-[34px]",
        "tablet:min-h-[34px]",
        "tablet:[&_svg]:w-[22px]",
        "tablet:[&_svg]:h-[22px]",
    ],
}
