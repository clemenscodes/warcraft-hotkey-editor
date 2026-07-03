use crate::{classes, states};
use warcraft_api::Race;

const BASE: &[&str] = &[
    "relative",
    "z-[2]",
    "py-[0.4rem]",
    "px-[0.6rem]",
    "pb-[0.5rem]",
    "w-full",
    "text-white",
    "min-w-0",
];
const MOBILE: &[&str] = &[
    "mobile:pt-[0.3rem]",
    "mobile:px-[0.15rem]",
    "mobile:pb-[0.45rem]",
    "mobile:text-[clamp(9px,2.4vw,13px)]",
    "mobile:tracking-[0.03em]",
];
const TABLET: &[&str] = &[
    "tablet:pt-[0.3rem]",
    "tablet:px-[0.15rem]",
    "tablet:pb-[0.45rem]",
    "tablet:text-[clamp(9px,2.4vw,13px)]",
    "tablet:tracking-[0.03em]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const HUMAN: &[&str] = &["group-data-[active=true]:text-race-human"];
const ORC: &[&str] = &["group-data-[active=true]:text-race-orc"];
const NIGHTELF: &[&str] = &["group-data-[active=true]:text-race-nightelf"];
const UNDEAD: &[&str] = &["group-data-[active=true]:text-race-undead"];
const NEUTRAL: &[&str] = &["group-data-[active=true]:text-warcraft-gold"];
states! {
    Race, Human => HUMAN, Orc => ORC, Nightelf => NIGHTELF, Undead => UNDEAD, Neutral => NEUTRAL
}
