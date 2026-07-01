use crate::classes;
const BASE: &[&str] = &[
    "group",
    "flex",
    "flex-col",
    "items-center",
    "gap-[0.2rem]",
    "max-w-full",
    "min-w-0",
    "m-0",
    "p-0",
    "bg-transparent",
    "border-none",
    "[color:inherit]",
    "[font:inherit]",
    "text-center",
    "cursor-default",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
