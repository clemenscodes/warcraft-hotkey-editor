use crate::{classes, states};

use super::state::UnitCardIdState;

// The unit's database id in a monospace face. Ellipsized single line. Grey normally;
// tinted with the inherited race color when the card is selected.
const BASE: &[&str] = &[
    "font-mono",
    "text-[1.05rem]",
    "leading-[1.2]",
    "overflow-hidden",
    "text-ellipsis",
    "whitespace-nowrap",
];
const MOBILE: &[&str] = &[
    "mobile:block",
    "mobile:w-full",
    "mobile:text-[11px]",
    "mobile:leading-[1.2]",
];
const TABLET: &[&str] = &[
    "tablet:block",
    "tablet:w-full",
    "tablet:text-[11px]",
    "tablet:leading-[1.2]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }

const NORMAL: &[&str] = &["text-[#7b818d]"];
const SELECTED: &[&str] = &["text-[color:var(--race-color,#c0a67c)]", "opacity-70"];

states! { UnitCardIdState, Normal => NORMAL, Selected => SELECTED }
