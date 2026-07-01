use crate::classes;

// The category tab row. Hidden on the sidebar (collapsible headings replace it);
// shown as a full-width equal-slot row on small screens.
const BASE: &[&str] = &["hidden"];

const MOBILE: &[&str] = &[
    "mobile:flex",
    "mobile:flex-row",
    "mobile:flex-nowrap",
    "mobile:gap-[0.4rem]",
    "mobile:w-full",
    "mobile:min-w-0",
    "mobile:p-0",
    "mobile:m-0",
];

const TABLET: &[&str] = &[
    "tablet:flex",
    "tablet:flex-row",
    "tablet:flex-nowrap",
    "tablet:gap-[0.4rem]",
    "tablet:w-full",
    "tablet:min-w-0",
    "tablet:p-0",
    "tablet:m-0",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
