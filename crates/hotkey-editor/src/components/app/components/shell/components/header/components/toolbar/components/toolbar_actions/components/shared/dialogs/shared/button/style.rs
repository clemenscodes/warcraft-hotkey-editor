use super::props::ButtonVariant;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "inline-flex",
    "items-center",
    "justify-center",
    "px-14",
    "py-6",
    "rounded-lg",
    "text-[2rem]",
    "whitespace-nowrap",
    "cursor-pointer",
    "select-none",
    "[transition:all_120ms]",
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

const PRIMARY: &[TailwindClass] = tw![
    "border",
    "border-warcraft-gold",
    "bg-panel-blue-solid",
    "text-warcraft-gold",
    "text-shadow-drop-92",
    "hover:bg-panel-blue-solid-2",
    "hover:shadow-glow-12-2",
];

const SECONDARY: &[TailwindClass] = tw![
    "border",
    "border-warcraft-blue",
    "bg-warcraft-bg-panel/70",
    "text-warcraft-text-secondary",
    "text-shadow-drop-60",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "hover:shadow-glow-12-3",
];
states! {
    ButtonVariant, Primary => PRIMARY, Secondary => SECONDARY
}
