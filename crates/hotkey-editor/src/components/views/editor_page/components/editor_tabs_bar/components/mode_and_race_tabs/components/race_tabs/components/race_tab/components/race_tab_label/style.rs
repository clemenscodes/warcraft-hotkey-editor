use crate::{classes, states, styling::TailwindClass, tw};
use warcraft_api::Race;

const BASE: &[TailwindClass] = tw![
    "relative",
    "z-[2]",
    "py-[0.4rem]",
    "px-[0.6rem]",
    "pb-[0.5rem]",
    "w-full",
    "text-white",
    "min-w-0",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:pt-[0.3rem]",
    "mobile:px-[0.15rem]",
    "mobile:pb-[0.45rem]",
    "mobile:text-[clamp(9px,2.4vw,13px)]",
    "mobile:tracking-[0.03em]",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:pt-[0.3rem]",
    "tablet:px-[0.15rem]",
    "tablet:pb-[0.45rem]",
    "tablet:text-[clamp(9px,2.4vw,13px)]",
    "tablet:tracking-[0.03em]",
];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const HUMAN: &[TailwindClass] = tw!["group-data-[active=true]:text-race-human"];
const ORC: &[TailwindClass] = tw!["group-data-[active=true]:text-race-orc"];
const NIGHTELF: &[TailwindClass] = tw!["group-data-[active=true]:text-race-nightelf"];
const UNDEAD: &[TailwindClass] = tw!["group-data-[active=true]:text-race-undead"];
const NEUTRAL: &[TailwindClass] = tw!["group-data-[active=true]:text-warcraft-gold"];
states! {
    Race, Human => HUMAN, Orc => ORC, Nightelf => NIGHTELF, Undead => UNDEAD, Neutral => NEUTRAL
}
