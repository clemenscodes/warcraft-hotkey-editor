use crate::classes;

const BASE: &[&str] = &[
    "absolute",
    "top-[0.4rem]",
    "right-[0.45rem]",
    "font-mono",
    "font-bold",
    "leading-none",
    "text-base",
    "text-[#ffe39a]",
    "pointer-events-none",
    "[text-shadow:1px_1px_0_rgba(0,0,0,0.95),-1px_1px_0_rgba(0,0,0,0.95),1px_-1px_0_rgba(0,0,0,0.95),-1px_-1px_0_rgba(0,0,0,0.95),0_0_3px_rgba(0,0,0,0.95)]",
];

const MOBILE: &[&str] = &[
    "mobile:top-[5px]",
    "mobile:right-[6px]",
    "mobile:text-[0.9rem]",
];

const TABLET: &[&str] = &[
    "tablet:top-[5px]",
    "tablet:right-[6px]",
    "tablet:text-[0.9rem]",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
