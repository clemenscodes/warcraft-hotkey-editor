use crate::{classes, styling::TailwindClass, tw};

// The blue-edged block that describes an ability's off-state or upgraded form. Shared
// by the alt-state and upgrade sections.
const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "gap-[0.3rem]",
    "py-[0.7rem]",
    "pr-0",
    "pl-4",
    "bg-[rgba(8,18,35,0.55)]",
    "border-l-2",
    "border-[#6ec3ff]",
    "rounded-[4px_0_0_4px]",
    "text-[#c8d4ec]",
    "text-[1.4rem]",
    "leading-[1.5]",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
