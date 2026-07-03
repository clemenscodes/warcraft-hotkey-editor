use crate::classes;

const BASE: &[&str] = &[
    "list-none",
    "m-0",
    "p-0",
    "w-full",
    "max-w-[110rem]",
    "mx-auto",
    "flex",
    "flex-col",
];

const MOBILE: &[&str] = &["mobile:max-w-full", "mobile:[touch-action:pan-y]"];
const TABLET: &[&str] = &["tablet:max-w-full", "tablet:[touch-action:pan-y]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
