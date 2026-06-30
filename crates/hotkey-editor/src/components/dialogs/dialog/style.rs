use crate::classes;

const BASE: &[&str] = &[
    "fixed",
    "inset-0",
    "z-[1000]",
    "flex",
    "items-center",
    "justify-center",
    "p-8",
    "bg-black/70",
];
const MOBILE: &[&str] = &["mobile:p-0"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
