use crate::{classes, states};

use super::props::ButtonVariant;

// A WC3 action button in two weights. The shape is shared; the weight (primary =
// gold on a blue gradient, secondary = muted until hovered) is a state overlay.
const BASE: &[&str] = &[
    "inline-flex",
    "items-center",
    "justify-center",
    "px-14",
    "py-6",
    "rounded-lg",
    "font-friz-quadrata",
    "text-[2rem]",
    "whitespace-nowrap",
    "cursor-pointer",
    "select-none",
    "[transition:all_120ms]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }

const PRIMARY: &[&str] = &[
    "border",
    "border-warcraft-gold",
    "[background:linear-gradient(180deg,#2a5085_0%,#1a3a5c_100%)]",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_rgba(0,0,0,0.92)]",
    "hover:[background:linear-gradient(180deg,#356dac_0%,#1f4a72_100%)]",
    "hover:[box-shadow:0_0_12px_rgba(255,206,99,0.4)]",
];
const SECONDARY: &[&str] = &[
    "border",
    "border-warcraft-blue",
    "bg-[rgba(20,40,70,0.7)]",
    "text-warcraft-text-secondary",
    "[text-shadow:1px_1px_0_rgba(0,0,0,0.6)]",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "hover:[box-shadow:0_0_12px_rgba(255,206,99,0.25)]",
];

states! { ButtonVariant, Primary => PRIMARY, Secondary => SECONDARY }
