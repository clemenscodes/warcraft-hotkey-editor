use crate::components::views::resolve_page::logic::ReasonKind;
use crate::states;
use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex-none",
    "inline-flex",
    "items-center",
    "px-[0.75rem]",
    "py-[0.3rem]",
    "rounded-[6px]",
    "font-friz-quadrata",
    "text-[1.35rem]",
    "uppercase",
    "[letter-spacing:0.04em]",
    "border",
    "border-solid",
    "[text-shadow:1px_1px_0_#000]",
    "whitespace-nowrap",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }

const FIGHT: &[TailwindClass] = tw![
    "text-[#ff9a6a]",
    "border-[rgba(255,154,106,0.6)]",
    "bg-[rgba(255,122,122,0.12)]",
];
const GAPPULL: &[TailwindClass] = tw![
    "text-[#7bdca0]",
    "border-[rgba(123,220,160,0.6)]",
    "bg-[rgba(123,220,160,0.12)]",
];
const SPILL: &[TailwindClass] = tw![
    "text-[#6aa1ff]",
    "border-[rgba(106,161,255,0.6)]",
    "bg-[rgba(106,161,255,0.12)]",
];
const SWAP: &[TailwindClass] = tw![
    "text-[#c79bff]",
    "border-[rgba(199,155,255,0.6)]",
    "bg-[rgba(199,155,255,0.12)]",
];
const STUCK: &[TailwindClass] = tw![
    "text-[#ff9a6a]",
    "border-[rgba(255,154,106,0.6)]",
    "bg-[rgba(255,122,122,0.12)]",
];

states! {
    ReasonKind,
    Fight => FIGHT,
    GapPull => GAPPULL,
    Spill => SPILL,
    Swap => SWAP,
    Stuck => STUCK,
}
