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
    "mobile:flex-row",
    "mobile:items-start",
    "mobile:h-auto",
    "mobile:min-h-0",
    "mobile:gap-[12px]",
    "mobile:pb-[16px]",
    "mobile:overflow-visible",
    "mobile:text-left",
    "mobile:w-full",
    "mobile:min-w-0",
];
const TABLET: &[&str] = &[
    "tablet:flex",
    "tablet:flex-row",
    "tablet:items-start",
    "tablet:h-auto",
    "tablet:min-h-0",
    "tablet:gap-[12px]",
    "tablet:pb-[16px]",
    "tablet:overflow-visible",
    "tablet:text-left",
    "tablet:w-full",
    "tablet:min-w-0",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
