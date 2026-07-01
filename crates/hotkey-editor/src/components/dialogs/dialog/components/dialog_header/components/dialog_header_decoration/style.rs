use crate::classes;

const BASE: &[&str] = &[
    "block",
    "h-[2.4rem]",
    "w-auto",
    "flex-none",
    "[filter:drop-shadow(0_1px_0_rgba(0,0,0,0.7))]",
];

const MOBILE: &[&str] = &["mobile:w-[2rem]"];
const TABLET: &[&str] = &["tablet:w-[2.75rem]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
