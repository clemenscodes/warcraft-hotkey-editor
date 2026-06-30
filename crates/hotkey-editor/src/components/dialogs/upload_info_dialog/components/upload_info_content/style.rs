use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "items-center",
    "justify-center",
    "gap-8",
    "flex-1",
    "w-full",
    "max-w-[70rem]",
    "mx-auto",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
