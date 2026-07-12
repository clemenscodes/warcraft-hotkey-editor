use tw_macro::tw;

// The collision card is a thin identity wrapper around its own `CollisionCardButton`
// button: it owns only the card's placement box (full width in the vertical sidebar; a
// fixed carousel item on mobile/tablet) and carries the `collision-card` identity class
// the deep-link e2e suite selects on. All of the card's look — border, fill, hover,
// focus, and the fixed gold selected accent — lives on the surface.

classes! {
    base: tw![
        "w-full",
    ],
    mobile: tw![
        "mobile:flex-[1_0_auto]",
        "mobile:w-[min(54cqi,260px)]",
        "mobile:h-full",
        "mobile:snap-start",
    ],
    tablet: tw![
        "tablet:flex-[1_0_auto]",
        "tablet:w-[min(54cqi,260px)]",
        "tablet:h-full",
        "tablet:snap-start",
    ],
}
