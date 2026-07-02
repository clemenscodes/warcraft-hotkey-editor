use crate::classes;

// A horizontal strip holding the mode toggle and race tabs, with a clamped height so
// the banners keep a consistent size. On phones it stacks into a column and drops the
// min-height.
const BASE: &[&str] = &[
    "flex",
    "items-stretch",
    "flex-none",
    "gap-6",
    "min-h-[clamp(9rem,13vh,18rem)]",
];
const MOBILE: &[&str] = &["mobile:flex-col", "mobile:min-h-0", "mobile:gap-[0.85rem]"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
