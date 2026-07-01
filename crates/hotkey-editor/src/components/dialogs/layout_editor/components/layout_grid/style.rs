use crate::classes;

const BASE: &[&str] = &[
    "grid",
    "[grid-template-columns:repeat(4,clamp(7rem,9vh,12rem))]",
    "[grid-auto-rows:clamp(7rem,9vh,12rem)]",
    "gap-[1.25rem]",
    "mx-auto",
];
const MOBILE: &[&str] = &[
    "mobile:justify-center",
    "mobile:[grid-template-columns:repeat(4,clamp(52px,18vw,72px))]",
    "mobile:[grid-auto-rows:clamp(52px,18vw,72px)]",
    "mobile:gap-[8px]",
];
const TABLET: &[&str] = &[
    "tablet:justify-center",
    "tablet:[grid-template-columns:repeat(4,clamp(52px,18vw,72px))]",
    "tablet:[grid-auto-rows:clamp(52px,18vw,72px)]",
    "tablet:gap-[8px]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
