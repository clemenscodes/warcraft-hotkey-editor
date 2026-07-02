use crate::classes;
const BASE: &[&str] = &[
    "flex",
    "items-center",
    "gap-3",
    "flex-none",
    "py-[1.2rem]",
    "border-b",
    "border-[rgba(255,206,99,0.25)]",
];
const MOBILE: &[&str] = &[
    "mobile:gap-[0.85rem]",
    "mobile:flex-nowrap",
    "mobile:overflow-x-auto",
    "mobile:overflow-y-hidden",
    "mobile:overscroll-x-contain",
    "mobile:snap-x",
    "mobile:snap-mandatory",
    "mobile:[scrollbar-width:none]",
    "mobile:[&::-webkit-scrollbar]:hidden",
];
const TABLET: &[&str] = &[
    "tablet:gap-[0.85rem]",
    "tablet:flex-nowrap",
    "tablet:overflow-x-auto",
    "tablet:overflow-y-hidden",
    "tablet:overscroll-x-contain",
    "tablet:snap-x",
    "tablet:snap-mandatory",
    "tablet:[scrollbar-width:none]",
    "tablet:[&::-webkit-scrollbar]:hidden",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
