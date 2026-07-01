use crate::classes;

const BASE: &[&str] = &[
    "text-[#d6dcec]",
    "font-medium",
    "[font-variant-numeric:tabular-nums]",
    "flex-[0_0_auto]",
    "group-data-[matchup=strong]:text-[#4ade80]",
    "group-data-[matchup=weak]:text-[#f87171]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
