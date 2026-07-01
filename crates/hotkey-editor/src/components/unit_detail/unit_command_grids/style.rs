use crate::classes;

const BASE: &[&str] = &[
    "grid",
    "grid-cols-[repeat(2,minmax(0,1fr))]",
    "gap-x-[clamp(1.5rem,1.4vw,2rem)]",
    "gap-y-[clamp(1rem,1.2vh,1.35rem)]",
    "items-start",
    "flex-none",
];
const MOBILE: &[&str] = &[
    "mobile:flex",
    "mobile:flex-col",
    "mobile:items-center",
    "mobile:gap-6",
];
const TABLET: &[&str] = &[
    "tablet:grid-cols-[repeat(2,1fr)]",
    "tablet:gap-x-[2.5rem]",
    "tablet:gap-y-[1.75rem]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
