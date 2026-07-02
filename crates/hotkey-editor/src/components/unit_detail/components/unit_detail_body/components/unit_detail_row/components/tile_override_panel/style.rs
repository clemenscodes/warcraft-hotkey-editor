use crate::classes;

// The right column of the unit-detail row: the "Hotkey override" heading and the
// override card (or its empty placeholder). On phones it becomes a sticky bottom
// sheet so the override stays reachable while the grid scrolls.
const BASE: &[&str] = &["flex", "flex-col", "min-w-0", "self-start"];
const MOBILE: &[&str] = &[
    "mobile:self-stretch",
    "mobile:items-stretch",
    "mobile:sticky",
    "mobile:bottom-0",
    "mobile:z-[100]",
    "mobile:m-[0_-0.85rem]",
    "mobile:pt-0",
];
const TABLET: &[&str] = &["tablet:self-stretch", "tablet:pt-0"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
