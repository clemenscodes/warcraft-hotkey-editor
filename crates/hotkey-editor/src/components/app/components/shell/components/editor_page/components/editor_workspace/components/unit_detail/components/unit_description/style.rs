use crate::{classes, styling::TailwindClass, tw};

// The unit's flavor text under the header. Reserves two lines' height on desktop so
// the stats card below never shifts; clamps to a single line on smaller panels.
const BASE: &[TailwindClass] = tw![
    "mt-4",
    "min-h-[9rem]",
    "text-[1.75rem]",
    "leading-[1.45]",
    "text-warcraft-text-secondary",
    "text-shadow-drop-60",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:mt-[12px]",
    "mobile:flex-none",
    "mobile:h-[1.4em]",
    "mobile:min-h-[1.4em]",
    "mobile:max-h-[1.4em]",
    "mobile:max-w-full",
    "mobile:text-[14px]",
    "mobile:leading-[1.4]",
    "mobile:line-clamp-1",
    "mobile:[overflow-wrap:break-word]",
    "mobile:[word-break:break-word]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:mt-[12px]",
    "tablet:flex-none",
    "tablet:h-[1.4em]",
    "tablet:min-h-[1.4em]",
    "tablet:max-h-[1.4em]",
    "tablet:max-w-full",
    "tablet:text-[14px]",
    "tablet:leading-[1.4]",
    "tablet:line-clamp-1",
    "tablet:[overflow-wrap:break-word]",
    "tablet:[word-break:break-word]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
