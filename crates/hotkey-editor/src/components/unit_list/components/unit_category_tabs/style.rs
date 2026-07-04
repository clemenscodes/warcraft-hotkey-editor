use crate::{classes, styling::TailwindClass, tw};

// The category tab row. Hidden on the sidebar (collapsible headings replace it);
// shown as a full-width equal-slot row on small screens.
const BASE: &[TailwindClass] = tw!["hidden"];

const MOBILE: &[TailwindClass] = tw![
    "mobile:flex",
    "mobile:flex-row",
    "mobile:flex-nowrap",
    "mobile:gap-[0.4rem]",
    "mobile:w-full",
    "mobile:min-w-0",
    "mobile:p-0",
    "mobile:m-0",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:flex",
    "tablet:flex-row",
    "tablet:flex-nowrap",
    "tablet:gap-[0.4rem]",
    "tablet:w-full",
    "tablet:min-w-0",
    "tablet:p-0",
    "tablet:m-0",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
