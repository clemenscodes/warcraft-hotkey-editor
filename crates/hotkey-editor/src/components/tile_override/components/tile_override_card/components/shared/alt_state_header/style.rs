use crate::classes;

// The top row of the alt-state block: label on the left, the position button and key
// cell on the right.
const BASE: &[&str] = &[
    "grid",
    "grid-cols-[minmax(0,1fr)_auto_auto]",
    "items-center",
    "gap-x-[0.85rem]",
];

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
