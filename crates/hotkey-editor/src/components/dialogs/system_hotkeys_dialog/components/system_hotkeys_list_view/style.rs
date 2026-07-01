use crate::classes;

// A centred column of hotkey rows, capped so the binding names and key chips stay
// on one line on desktop.
const BASE: &[&str] = &[
    "list-none",
    "m-0",
    "p-0",
    "w-full",
    "max-w-[110rem]",
    "mx-auto",
    "flex",
    "flex-col",
];
const MOBILE: &[&str] = &["mobile:max-w-full", "mobile:[touch-action:pan-y]"];
const TABLET: &[&str] = &["tablet:max-w-full", "tablet:[touch-action:pan-y]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
