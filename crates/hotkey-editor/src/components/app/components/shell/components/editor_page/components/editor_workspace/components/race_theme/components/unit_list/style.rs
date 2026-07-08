use tw_macro::{ClassList, tw};
// The unit list panel. On the sidebar it is an absolutely-positioned column filling
// the sidebar column of the editor workspace; on small screens it collapses to a
// static, self-contained block (search + tabs + horizontal card carousel). `group`
// lets the cards read the active category and the scrollbar reveal on hover. The
// per-band sidebar widths match the workspace grid's first column.

classes! {
    base: tw![
        "group",
        "flex",
        "flex-col",
        "gap-2",
        "overflow-hidden",
        "min-w-0",
        "min-h-0",
    ],
    mobile: tw![
        "mobile:static",
        "mobile:w-full",
        "mobile:max-h-none",
        "mobile:overflow-visible",
        "mobile:gap-4",
        "mobile:p-0",
        "mobile:bg-transparent",
        "mobile:border-0",
        "mobile:contain-[layout]",
    ],
    tablet: tw![
        "tablet:static",
        "tablet:w-full",
        "tablet:max-h-none",
        "tablet:overflow-visible",
        "tablet:gap-4",
        "tablet:p-0",
        "tablet:bg-transparent",
        "tablet:border-0",
        "tablet:contain-[layout]",
    ],
    laptop: tw![
        "laptop:absolute",
        "laptop:top-0",
        "laptop:left-0",
        "laptop:w-136",
        "laptop:h-full",
    ],
    desktop: tw![
        "desktop:absolute",
        "desktop:top-0",
        "desktop:left-0",
        "desktop:w-136",
        "desktop:h-full",
    ],
    qhd: tw![
        "qhd:absolute",
        "qhd:top-0",
        "qhd:left-0",
        "qhd:w-184",
        "qhd:h-full",
    ],
    uhd: tw![
        "uhd:absolute",
        "uhd:top-0",
        "uhd:left-0",
        "uhd:w-248",
        "uhd:h-full",
    ],
}

/// The mobile/tablet category tab row (hidden on the sidebar). Its own inlined
/// element in the unit list; sub-elements carry no shared identity component.
pub(super) const TABS: ClassList = ClassList::new(
    "hidden mobile:flex mobile:flex-row mobile:flex-nowrap mobile:gap-1.5 mobile:w-full mobile:min-w-0 mobile:p-0 mobile:m-0 tablet:flex tablet:flex-row tablet:flex-nowrap tablet:gap-1.5 tablet:w-full tablet:min-w-0 tablet:p-0 tablet:m-0",
);
/// The scroll region around the section track: vertical on the sidebar (gold
/// scrollbar revealed by the list group's hover), horizontal snap carousel on small
/// screens.
pub(super) const SCROLL: ClassList = ClassList::new(
    "flex-1 flex flex-col min-h-0 overflow-y-auto overflow-x-hidden pr-1 scrollbar-thin [scrollbar-color:color-mix(in_oklab,var(--color-warcraft-gold)_0%,transparent)_transparent] transition-[scrollbar-color] duration-slow group-hover:[scrollbar-color:color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)_transparent] [&::-webkit-scrollbar]:w-1.5 [&::-webkit-scrollbar-track]:bg-transparent [&::-webkit-scrollbar-thumb]:bg-transparent [&::-webkit-scrollbar-thumb]:rounded-hairline group-hover:[&::-webkit-scrollbar-thumb]:bg-warcraft-gold/45 [&::-webkit-scrollbar-thumb:hover]:bg-warcraft-gold mobile:overflow-x-auto mobile:overflow-y-hidden mobile:max-h-none mobile:pr-0 mobile:pb-1 mobile:flex-none mobile:@container mobile:h-28 mobile:[-webkit-overflow-scrolling:touch] mobile:overscroll-x-contain mobile:[scroll-snap-type:x_mandatory] mobile:scrollbar-none mobile:scroll-ps-[0.4rem] mobile:[&::-webkit-scrollbar]:hidden tablet:overflow-x-auto tablet:overflow-y-hidden tablet:max-h-none tablet:pr-0 tablet:pb-1 tablet:flex-none tablet:@container tablet:h-30 tablet:[-webkit-overflow-scrolling:touch] tablet:overscroll-x-contain tablet:[scroll-snap-type:x_mandatory] tablet:scrollbar-none tablet:scroll-ps-[0.4rem] tablet:[&::-webkit-scrollbar]:hidden",
);
/// The inner track laying out the category sections: a vertical stack on the sidebar,
/// a horizontal fixed-height carousel on small screens.
pub(super) const TRACK: ClassList = ClassList::new(
    "flex flex-col gap-2 mobile:flex-row mobile:flex-nowrap mobile:w-max mobile:min-w-full mobile:items-stretch mobile:h-full mobile:px-1.5 mobile:py-0 tablet:flex-row tablet:flex-nowrap tablet:w-max tablet:min-w-full tablet:items-stretch tablet:h-full tablet:px-1.5 tablet:py-0",
);
