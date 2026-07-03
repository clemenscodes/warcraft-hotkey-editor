use crate::classes;

// The unit's flavor text under the header. Reserves two lines' height on desktop so
// the stats card below never shifts; clamps to a single line on smaller panels.
const BASE: &[&str] = &[
    "mt-4",
    "min-h-[9rem]",
    "text-[1.75rem]",
    "leading-[1.45]",
    "text-[#c0c8da]",
    "[text-shadow:1px_1px_0_rgba(0,0,0,0.6)]",
];

const MOBILE: &[&str] = &[
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

const TABLET: &[&str] = &[
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

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
