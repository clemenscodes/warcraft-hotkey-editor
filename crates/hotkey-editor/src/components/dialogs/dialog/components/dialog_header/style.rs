use crate::classes;

const BASE: &[&str] = &[
    "relative",
    "flex",
    "items-center",
    "justify-center",
    "gap-6",
    "flex-none",
    "pt-[1.6rem]",
    "px-[4.5rem]",
    "pb-[1.4rem]",
    "border-b",
    "border-warcraft-gold-soft",
    "shadow-[0_1px_0_rgba(0,0,0,0.7),0_2px_0_rgba(255,206,99,0.1)]",
];
const MOBILE: &[&str] = &["mobile:gap-2", "mobile:px-[1.4rem]"];
const TABLET: &[&str] = &["tablet:gap-2", "tablet:px-[2rem]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
