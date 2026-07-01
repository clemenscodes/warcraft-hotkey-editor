use crate::classes;

// The chevron between category tabs on desktop; hidden inside the mobile popover.
const BASE: &[&str] = &[
    "font-friz-quadrata",
    "text-[2rem]",
    "leading-none",
    "select-none",
    "text-warcraft-gold/45",
    "group-data-[open=true]:hidden",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
