use crate::components::app::components::shell::components::toasts::ToastType;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex-none",
    "flex",
    "items-center",
    "justify-center",
    "w-12",
    "h-12",
    "rounded-full",
    "self-center",
    "[&>svg]:w-8",
    "[&>svg]:h-8",
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

const SUCCESS: &[TailwindClass] = tw![
    "[background-color:color-mix(in_oklab,var(--color-warcraft-success)_18%,transparent)]",
    "text-warcraft-success"
];
const ERROR: &[TailwindClass] = tw![
    "[background-color:color-mix(in_oklab,var(--color-race-orc)_20%,transparent)]",
    "text-race-orc"
];
const WARNING: &[TailwindClass] = tw![
    "[background-color:color-mix(in_oklab,var(--color-warcraft-gold)_20%,transparent)]",
    "text-warcraft-gold"
];
const INFO: &[TailwindClass] = tw![
    "[background-color:color-mix(in_oklab,var(--color-race-human)_18%,transparent)]",
    "text-race-human"
];
states! {
    ToastType, Success => SUCCESS, Error => ERROR, Warning => WARNING, Info => INFO
}
