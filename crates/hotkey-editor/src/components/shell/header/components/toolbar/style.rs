use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-row",
    "items-center",
    "justify-end",
    "gap-[0.65rem]",
    "min-w-0",
];

const MOBILE: &[&str] = &["mobile:gap-1"];
const TABLET: &[&str] = &["tablet:gap-1"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
