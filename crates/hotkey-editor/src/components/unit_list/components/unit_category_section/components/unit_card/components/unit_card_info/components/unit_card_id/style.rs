use super::state::UnitCardIdState;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "font-mono",
    "text-[1.05rem]",
    "leading-[1.2]",
    "overflow-hidden",
    "text-ellipsis",
    "whitespace-nowrap",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:block",
    "mobile:w-full",
    "mobile:text-[11px]",
    "mobile:leading-[1.2]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:block",
    "tablet:w-full",
    "tablet:text-[11px]",
    "tablet:leading-[1.2]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const NORMAL: &[TailwindClass] = tw!["text-[#7b818d]"];
// Selected: the id text takes the card's race accent (chosen off the `data-race`
// attribute the component renders), at reduced opacity.
const SELECTED: &[TailwindClass] = tw![
    "text-[color:#c0a67c]",
    "opacity-70",
    "data-[race=human]:text-[color:#6aa1ff]",
    "data-[race=orc]:text-[color:#ff7a7a]",
    "data-[race=nightelf]:text-[color:#5fdada]",
    "data-[race=undead]:text-[color:#c79bff]",
    "data-[race=neutral]:text-[color:#ffce63]",
];
states! {
    UnitCardIdState, Normal => NORMAL, Selected => SELECTED
}
