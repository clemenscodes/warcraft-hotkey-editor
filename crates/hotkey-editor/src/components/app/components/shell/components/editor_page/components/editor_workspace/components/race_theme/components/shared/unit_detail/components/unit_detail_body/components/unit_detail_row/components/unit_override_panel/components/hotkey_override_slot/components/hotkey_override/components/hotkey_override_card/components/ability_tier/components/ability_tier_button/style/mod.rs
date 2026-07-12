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
        "rounded-control",
        "cursor-pointer",
        "transition-[border-color,background]",
        "duration-fast",
        "hover:border-warcraft-gold",
        "hover:bg-warcraft-gold/12",
        "kb-focus:outline-none",
        "kb-focus:shadow-focus",
        "[&>span]:block",
        "[&_svg]:w-[1.7rem]",
        "[&_svg]:h-[1.7rem]",
    ],
    mobile: tw![
        "mobile:w-8.5",
        "mobile:h-8.5",
        "mobile:min-w-8.5",
        "mobile:min-h-8.5",
        "mobile:[&_svg]:w-5.5",
        "mobile:[&_svg]:h-5.5",
    ],
    tablet: tw![
        "tablet:w-8.5",
        "tablet:h-8.5",
        "tablet:min-w-8.5",
        "tablet:min-h-8.5",
        "tablet:[&_svg]:w-5.5",
        "tablet:[&_svg]:h-5.5",
    ],
}
