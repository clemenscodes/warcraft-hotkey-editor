use crate::{classes, styling::TailwindClass, tw};

// The ability / unit name heading in the override panel. Gold display face, ellipsized
// on one line; smaller on the mobile panel.
const BASE: &[TailwindClass] = tw![
    "m-0",
    "max-w-full",
    "overflow-hidden",
    "whitespace-nowrap",
    "text-ellipsis",
    "font-normal",
    "text-[2rem]",
    "leading-[1.2]",
    "text-warcraft-gold",
    "text-shadow-drop-92",
];

const MOBILE: &[TailwindClass] = tw!["mobile:text-[15px]", "mobile:[word-break:normal]"];
const TABLET: &[TailwindClass] = tw!["tablet:text-[15px]", "tablet:[word-break:normal]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
