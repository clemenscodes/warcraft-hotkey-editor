use crate::components::app::components::shell::components::resolve_page::logic::ReasonKind;
use crate::states;
use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex-none",
    "inline-flex",
    "items-center",
    "px-[0.75rem]",
    "py-[0.3rem]",
    "rounded-[6px]",
    "text-[1.35rem]",
    "uppercase",
    "[letter-spacing:0.04em]",
    "border",
    "border-solid",
    "text-shadow-drop",
    "whitespace-nowrap",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }

const FIGHT: &[TailwindClass] = tw!["text-race-orc", "border-race-orc/60", "bg-race-orc/12",];
const GAPPULL: &[TailwindClass] = tw![
    "text-warcraft-success",
    "border-warcraft-success/60",
    "bg-warcraft-success/12",
];
const SPILL: &[TailwindClass] = tw![
    "text-race-human",
    "border-race-human/60",
    "bg-race-human/12",
];
const SWAP: &[TailwindClass] = tw![
    "text-race-undead",
    "border-race-undead/60",
    "bg-race-undead/12",
];
const STUCK: &[TailwindClass] = tw!["text-race-orc", "border-race-orc/60", "bg-race-orc/12",];

states! {
    ReasonKind,
    Fight => FIGHT,
    GapPull => GAPPULL,
    Spill => SPILL,
    Swap => SWAP,
    Stuck => STUCK,
}
