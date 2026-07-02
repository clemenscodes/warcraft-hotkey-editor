use crate::classes;
const BASE: &[&str] = &[
    "flex-[1_1_0]",
    "min-h-0",
    "overflow-y-auto",
    "p-[0.75rem_0.75rem_0.75rem_0]",
    "[scrollbar-width:thin]",
    "[scrollbar-color:rgba(255,206,99,0.45)_transparent]",
];
const MOBILE: &[&str] = &[
    "mobile:flex-none",
    "mobile:min-h-[auto]",
    "mobile:overflow-y-visible",
    "mobile:p-[0.75rem_0]",
];
const TABLET: &[&str] = &[
    "tablet:flex-none",
    "tablet:min-h-[auto]",
    "tablet:overflow-y-visible",
    "tablet:p-[0.75rem_0]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
