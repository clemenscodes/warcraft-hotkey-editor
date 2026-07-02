use crate::classes;
const BASE: &[&str] = &[
    "grid",
    "grid-cols-[repeat(auto-fill,minmax(min(760px,100%),1fr))]",
    "gap-4",
    "content-start",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
