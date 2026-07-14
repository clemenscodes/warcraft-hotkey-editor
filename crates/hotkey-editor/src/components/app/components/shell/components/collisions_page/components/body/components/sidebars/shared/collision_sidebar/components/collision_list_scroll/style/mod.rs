use tw_macro::tw;
// The scroll region around the collision cards. Vertical scroll on the sidebar
// (the shell's global gold scrollbar applies); horizontal snap scroll (the
// swipe carousel) on small screens with the scrollbar hidden.

classes! {
    base: tw![
        "flex-1",
        "flex",
        "flex-col",
        "min-h-0",
        "overflow-y-auto",
        "overflow-x-hidden",
        "pr-1",
    ],
    mobile: tw![
        "mobile:overflow-x-auto",
        "mobile:overflow-y-hidden",
        "mobile:max-h-none",
        "mobile:pr-0",
        "mobile:pb-1",
        "mobile:flex-none",
        "mobile:@container",
        "mobile:h-28",
        "mobile:[-webkit-overflow-scrolling:touch]",
        "mobile:overscroll-x-contain",
        "mobile:snap-x",
        "mobile:snap-mandatory",
        "mobile:scrollbar-none",
        "mobile:scroll-ps-[0.4rem]",
        "mobile:[&::-webkit-scrollbar]:hidden",
    ],
    tablet: tw![
        "tablet:overflow-x-auto",
        "tablet:overflow-y-hidden",
        "tablet:max-h-none",
        "tablet:pr-0",
        "tablet:pb-1",
        "tablet:flex-none",
        "tablet:@container",
        "tablet:h-30",
        "tablet:[-webkit-overflow-scrolling:touch]",
        "tablet:overscroll-x-contain",
        "tablet:snap-x",
        "tablet:snap-mandatory",
        "tablet:scrollbar-none",
        "tablet:scroll-ps-[0.4rem]",
        "tablet:[&::-webkit-scrollbar]:hidden",
    ],
}
