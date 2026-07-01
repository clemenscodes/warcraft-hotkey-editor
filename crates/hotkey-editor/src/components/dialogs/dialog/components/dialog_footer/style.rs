use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "items-center",
    "justify-end",
    "flex-none",
    "gap-4",
    "pt-[1.4rem]",
    "px-[4.5rem]",
    "pb-[1.8rem]",
    "border-t",
    "border-warcraft-gold-soft",
];

const MOBILE: &[&str] = &["mobile:justify-center", "mobile:px-[1.5rem]"];
const TABLET: &[&str] = &["tablet:justify-center"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
