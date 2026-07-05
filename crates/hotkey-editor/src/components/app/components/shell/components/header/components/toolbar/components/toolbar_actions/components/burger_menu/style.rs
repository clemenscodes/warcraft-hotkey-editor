use crate::{classes, styling::TailwindClass, tw};

// The burger appears at a single size (the compact header, hidden at laptop and up),
// so its chrome is fixed rather than cqi — there is no size change to scale against.
const BASE: &[TailwindClass] = tw![
    "inline-flex",
    "items-center",
    "justify-center",
    "shrink-0",
    "self-center",
    "p-0",
    "w-9",
    "h-9",
    "min-w-9",
    "min-h-9",
    "[background:linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold-dark)_55%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_55%,transparent)_100%)]",
    "border",
    "border-warcraft-gold-border",
    "rounded-[5.4px]",
    "text-warcraft-text-secondary",
    "cursor-pointer",
    "[transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease]",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "hover:[background:linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold-dark)_55%,transparent)_100%)]",
    "hover:[box-shadow:0_0_12px_color-mix(in_oklab,var(--color-warcraft-gold)_30%,transparent)]",
    "focus:outline-none",
    "focus-visible:border-white",
    "focus-visible:text-white",
    "focus-visible:[box-shadow:0_0_0_3px_var(--color-warcraft-highlight),0_0_16px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent)]",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw!["laptop:hidden"];
const DESKTOP: &[TailwindClass] = tw!["desktop:hidden"];
const QHD: &[TailwindClass] = tw!["qhd:hidden"];
const UHD: &[TailwindClass] = tw!["uhd:hidden"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
