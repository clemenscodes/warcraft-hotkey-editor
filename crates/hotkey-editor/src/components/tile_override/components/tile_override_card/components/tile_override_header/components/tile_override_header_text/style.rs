use crate::classes;

// The name-and-id column of the override header; centered with a touch floor on the
// mobile panel so the row height stays stable.
const BASE: &[&str] = &["flex", "flex-col", "items-start", "gap-[0.4rem]", "min-w-0"];

const MOBILE: &[&str] = &[
    "mobile:gap-[2px]",
    "mobile:min-h-[44px]",
    "mobile:justify-center",
    "mobile:text-left",
];

const TABLET: &[&str] = &[
    "tablet:gap-[2px]",
    "tablet:min-h-[44px]",
    "tablet:justify-center",
    "tablet:text-left",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
