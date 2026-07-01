use crate::classes;

// The ability / unit name heading in the override panel. Gold display face, ellipsized
// on one line; smaller on the mobile panel.
const BASE: &[&str] = &[
    "m-0",
    "max-w-full",
    "overflow-hidden",
    "whitespace-nowrap",
    "text-ellipsis",
    "font-friz-quadrata",
    "font-normal",
    "text-[2rem]",
    "leading-[1.2]",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_rgba(0,0,0,0.92)]",
];

const MOBILE: &[&str] = &["mobile:text-[15px]", "mobile:[word-break:normal]"];
const TABLET: &[&str] = &["tablet:text-[15px]", "tablet:[word-break:normal]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
