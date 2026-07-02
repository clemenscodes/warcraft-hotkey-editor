use crate::classes;
const BASE: &[&str] = &[
    "font-mono",
    "text-[1.6rem]",
    "opacity-80",
    "before:content-['(']",
    "after:content-[')']",
    "group-data-[active=true]:opacity-100",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
