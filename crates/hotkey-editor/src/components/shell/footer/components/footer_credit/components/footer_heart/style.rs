use crate::classes;

const BASE: &[&str] = &[
    "inline-flex",
    "items-center",
    "justify-center",
    "w-4",
    "h-4",
    "text-rose-400/90",
    "drop-shadow-[0_0_4px_rgba(244,114,182,0.35)]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
