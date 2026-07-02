use crate::classes;

const BASE: &[&str] = &[
    "grid",
    "grid-cols-[clamp(5.75rem,4.8vw,7.5rem)_1fr]",
    "items-start",
    "gap-x-[clamp(0.85rem,0.8vw,1.25rem)]",
    "mb-0",
    "pb-[clamp(0.4rem,0.7vh,0.7rem)]",
    "border-b",
    "border-[#1f3d63]",
];
const MOBILE: &[&str] = &[
    "mobile:flex",
    "mobile:flex-col",
    "mobile:items-center",
    "mobile:text-center",
    "mobile:h-auto",
    "mobile:gap-[0.85rem]",
    "mobile:pb-[1rem]",
];
const TABLET: &[&str] = &[
    "tablet:grid-cols-[11.5rem_1fr]",
    "tablet:gap-x-[1.25rem]",
    "tablet:pb-[1rem]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
