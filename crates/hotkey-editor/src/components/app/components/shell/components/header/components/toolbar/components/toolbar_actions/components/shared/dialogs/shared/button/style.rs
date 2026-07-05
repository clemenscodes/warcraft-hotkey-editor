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
    "[background:linear-gradient(180deg,#2a5085_0%,#1a3a5c_100%)]",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_color-mix(in_oklab,var(--color-warcraft-shadow)_92%,transparent)]",
    "hover:[background:linear-gradient(180deg,#356dac_0%,#1f4a72_100%)]",
    "hover:[box-shadow:0_0_12px_color-mix(in_oklab,var(--color-warcraft-gold)_40%,transparent)]",
];

const SECONDARY: &[TailwindClass] = tw![
    "border",
    "border-warcraft-blue",
    "bg-warcraft-bg-panel/70",
    "text-warcraft-text-secondary",
    "[text-shadow:1px_1px_0_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent)]",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "hover:[box-shadow:0_0_12px_color-mix(in_oklab,var(--color-warcraft-gold)_25%,transparent)]",
];
states! {
    ButtonVariant, Primary => PRIMARY, Secondary => SECONDARY
}
