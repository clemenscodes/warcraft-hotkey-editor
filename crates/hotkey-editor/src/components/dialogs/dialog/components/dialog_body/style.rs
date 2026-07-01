use crate::classes;

const BASE: &[&str] = &[
    "flex-1",
    "min-h-0",
    "flex",
    "flex-col",
    "gap-6",
    "pt-[2.4rem]",
    "px-[3rem]",
    "pb-[2.6rem]",
    "overflow-y-auto",
];
const MOBILE: &[&str] = &[
    "mobile:pt-[1.25rem]",
    "mobile:px-[1rem]",
    "mobile:pb-[1.5rem]",
];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
