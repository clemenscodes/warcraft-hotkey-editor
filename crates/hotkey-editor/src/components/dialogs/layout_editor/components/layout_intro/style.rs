use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "items-center",
    "gap-[0.7rem]",
    "m-0",
    "text-center",
    "[text-shadow:1px_1px_0_#000]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
