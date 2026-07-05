use crate::components::app::components::shell::components::toasts::ToastType;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "text-warcraft-gold",
    "uppercase",
    "tracking-[0.06em]",
    "text-[1.9rem]",
    "leading-[1.2]",
    "text-shadow-drop",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const SUCCESS: &[TailwindClass] = tw!["text-warcraft-success"];
const ERROR: &[TailwindClass] = tw!["text-race-orc"];
const WARNING: &[TailwindClass] = tw!["text-warcraft-gold"];
const INFO: &[TailwindClass] = tw![];
states! {
    ToastType, Success => SUCCESS, Error => ERROR, Warning => WARNING, Info => INFO
}
