use crate::classes;

const BASE: &[&str] = &[
    "absolute",
    "top-[calc(100%+4px)]",
    "left-0",
    "z-[200]",
    "min-w-full",
    "flex",
    "flex-col",
    "gap-[2px]",
    "p-[4px]",
    "[background:linear-gradient(170deg,rgba(12,25,50,0.98)_0%,rgba(6,12,28,0.98)_100%)]",
    "border",
    "border-[rgba(255,206,99,0.45)]",
    "rounded-[8px]",
    "[box-shadow:0_8px_24px_rgba(0,0,0,0.65),0_0_12px_rgba(255,206,99,0.1)]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
