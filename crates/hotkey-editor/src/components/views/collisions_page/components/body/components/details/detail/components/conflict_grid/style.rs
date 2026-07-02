use crate::classes;
const BASE: &[&str] = &[
    "grid",
    "grid-cols-[repeat(auto-fill,minmax(450px,1fr))]",
    "gap-6",
    "flex-[1_1_0]",
    "min-h-0",
    "overflow-y-auto",
    "content-start",
    "p-[1rem_0.75rem_1rem_0]",
    "[scrollbar-width:thin]",
    "[scrollbar-color:rgba(255,206,99,0.45)_transparent]",
];
const MOBILE: &[&str] = &[
    "mobile:grid-cols-[minmax(0,1fr)]",
    "mobile:flex-none",
    "mobile:min-h-[auto]",
    "mobile:overflow-y-visible",
    "mobile:p-[1rem_0]",
];
const TABLET: &[&str] = &[
    "tablet:grid-cols-[minmax(0,1fr)]",
    "tablet:flex-none",
    "tablet:min-h-[auto]",
    "tablet:overflow-y-visible",
    "tablet:p-[1rem_0]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
