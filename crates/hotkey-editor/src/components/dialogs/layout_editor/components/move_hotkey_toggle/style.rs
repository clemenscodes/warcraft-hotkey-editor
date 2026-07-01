use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "items-center",
    "gap-[0.8rem]",
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.06em]",
    "text-[1.9rem]",
    "text-warcraft-gold",
    "cursor-pointer",
    "[text-shadow:1px_1px_0_#000]",
];
const MOBILE: &[&str] = &["mobile:gap-[8px]", "mobile:text-[15px]"];
const TABLET: &[&str] = &["tablet:gap-[8px]", "tablet:text-[15px]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
