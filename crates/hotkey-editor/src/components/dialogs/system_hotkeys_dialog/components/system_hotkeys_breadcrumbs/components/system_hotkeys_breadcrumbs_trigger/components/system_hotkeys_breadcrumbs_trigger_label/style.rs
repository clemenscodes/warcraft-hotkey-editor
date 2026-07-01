use crate::classes;

const BASE: &[&str] = &[];

const MOBILE: &[&str] = &[
    "mobile:[flex:1_1_auto]",
    "mobile:text-left",
    "mobile:whitespace-nowrap",
    "mobile:overflow-hidden",
    "mobile:text-ellipsis",
];

const TABLET: &[&str] = &[
    "tablet:[flex:1_1_auto]",
    "tablet:text-left",
    "tablet:whitespace-nowrap",
    "tablet:overflow-hidden",
    "tablet:text-ellipsis",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
