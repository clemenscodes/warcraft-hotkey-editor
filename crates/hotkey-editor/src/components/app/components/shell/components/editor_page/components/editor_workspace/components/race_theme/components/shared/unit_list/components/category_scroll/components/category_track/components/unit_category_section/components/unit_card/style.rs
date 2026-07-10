use tw_macro::tw;

// The unit card is a thin identity wrapper around its own `UnitCardSurface` button:
// it owns only the card's placement box (full width in the vertical list; a fixed
// carousel item on mobile/tablet, sized as a size-container so the portrait can scale
// in `cqh`). All of the card's look — border, fill, hover, focus, and the
// `--race-color` selected accent — lives on the surface.

classes! {
    base: tw!["w-full"],
    mobile: tw![
        "mobile:flex-[1_0_auto]",
        "mobile:w-[min(54cqi,260px)]",
        "mobile:h-full",
        "mobile:@container-size",
        "mobile:snap-start",
    ],
    tablet: tw![
        "tablet:flex-[1_0_auto]",
        "tablet:w-[min(54cqi,260px)]",
        "tablet:h-full",
        "tablet:@container-size",
        "tablet:snap-start",
    ],
}
