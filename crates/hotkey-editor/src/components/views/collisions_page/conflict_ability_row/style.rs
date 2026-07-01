use crate::classes;
const BASE: &[&str] = &[
    "grid",
    "grid-cols-[1fr_auto_1fr]",
    "items-start",
    "justify-items-center",
    "gap-[12px]",
    "w-full",
    "data-[multi=true]:grid-cols-none",
    "data-[multi=true]:flex",
    "data-[multi=true]:flex-wrap",
    "data-[multi=true]:justify-center",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
