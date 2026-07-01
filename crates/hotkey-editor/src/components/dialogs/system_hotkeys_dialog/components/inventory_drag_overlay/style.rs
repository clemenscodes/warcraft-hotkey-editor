use crate::classes;

const BASE: &[&str] = &[
    "fixed",
    "pointer-events-none",
    "z-[1100]",
    "flex",
    "items-center",
    "justify-center",
    "border-solid",
    "border-[12px]",
    "select-none",
    "[background:linear-gradient(180deg,rgba(15,22,45,0.95)_0%,rgba(8,14,30,0.98)_100%)]",
    "[border-image-source:var(--wc3-slot-frame)]",
    "[border-image-slice:12_fill]",
    "[border-image-repeat:stretch]",
    "[filter:drop-shadow(0_8px_24px_rgba(0,0,0,0.6))_drop-shadow(0_0_16px_rgba(255,206,99,0.6))]",
];

const MOBILE: &[&str] = &["mobile:border-[8px]"];
const TABLET: &[&str] = &["tablet:border-[8px]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
