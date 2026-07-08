use tw_macro::tw;

// The unit card is now a thin identity wrapper around the shared
// `SelectableEntityCard` surface: it owns only the card's placement box (full width in
// the vertical list; a fixed carousel item on mobile/tablet, sized as a size-container
// so the portrait can scale in `cqh`) and the per-kind carousel visibility filter,
// which reads the `data-unit-kind` attribute this wrapper renders. All of the card's
// look — border, fill, hover, focus, and the per-race selected accent — lives on the
// shared surface.

classes! {
    base: tw!["w-full"],
    mobile: tw![
        "mobile:flex-[1_0_auto]",
        "mobile:w-[min(54cqi,260px)]",
        "mobile:h-full",
        "mobile:@container-size",
        "mobile:snap-start",
        "mobile:group-[[data-search-active=false][data-active-category=hero]]:[&:not([data-unit-kind=hero])]:hidden",
        "mobile:group-[[data-search-active=false][data-active-category=soldier]]:[&:not([data-unit-kind=soldier])]:hidden",
        "mobile:group-[[data-search-active=false][data-active-category=worker]]:[&:not([data-unit-kind=worker])]:hidden",
        "mobile:group-[[data-search-active=false][data-active-category=building]]:[&:not([data-unit-kind=building])]:hidden",
    ],
    tablet: tw![
        "tablet:flex-[1_0_auto]",
        "tablet:w-[min(54cqi,260px)]",
        "tablet:h-full",
        "tablet:@container-size",
        "tablet:snap-start",
        "tablet:group-[[data-search-active=false][data-active-category=hero]]:[&:not([data-unit-kind=hero])]:hidden",
        "tablet:group-[[data-search-active=false][data-active-category=soldier]]:[&:not([data-unit-kind=soldier])]:hidden",
        "tablet:group-[[data-search-active=false][data-active-category=worker]]:[&:not([data-unit-kind=worker])]:hidden",
        "tablet:group-[[data-search-active=false][data-active-category=building]]:[&:not([data-unit-kind=building])]:hidden",
    ],
}
