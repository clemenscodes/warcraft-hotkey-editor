use crate::classes;

// The control-group strip: ten narrow cells across on desktop; five-by-two on
// small viewports so they stay tappable.
const BASE: &[&str] = &["grid", "grid-cols-[repeat(10,11rem)]", "gap-[0.8rem]"];
const MOBILE: &[&str] = &[
    "mobile:grid-cols-[repeat(5,minmax(0,1fr))]",
    "mobile:auto-rows-[minmax(72px,auto)]",
    "mobile:gap-[0.4rem]",
    "mobile:w-full",
];
const TABLET: &[&str] = &[
    "tablet:grid-cols-[repeat(5,minmax(0,1fr))]",
    "tablet:auto-rows-[minmax(72px,auto)]",
    "tablet:gap-[0.4rem]",
    "tablet:w-full",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
