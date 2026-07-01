use crate::classes;

// The tier-cycling footer: prev button, level caption, next button, centered and
// pushed to the bottom of the override card.
const BASE: &[&str] = &[
    "mt-auto",
    "flex",
    "items-center",
    "justify-center",
    "gap-[0.85rem]",
    "pt-4",
];

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
