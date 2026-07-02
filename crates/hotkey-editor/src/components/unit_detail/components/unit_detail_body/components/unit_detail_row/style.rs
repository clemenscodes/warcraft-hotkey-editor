use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "items-stretch",
    "px-0",
    "mt-0",
    "pt-0",
    "flex-none",
    "min-h-0",
    "gap-[clamp(0.95rem,1.6vh,1.5rem)]",
];
const MOBILE: &[&str] = &[
    "mobile:grid",
    "mobile:grid-cols-[minmax(0,1fr)]",
    "mobile:gap-4",
    "mobile:items-start",
    "mobile:mt-5",
];
const TABLET: &[&str] = &[
    "tablet:grid",
    "tablet:grid-cols-[minmax(0,1fr)]",
    "tablet:gap-10",
    "tablet:items-start",
    "tablet:mt-5",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
