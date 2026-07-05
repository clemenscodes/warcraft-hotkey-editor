use super::state::CollisionState;
use crate::{classes, states, styling::TailwindClass, tw};

// The button surface fills its host container and draws itself as a cqi-scaled drawing:
// every interior length resolves against the host box (the host is the query context).
// The border is tuned per band to read as a ~1px hairline: 2.8cqi on the 36px compact
// phone/tablet box, and 1.25cqi on the laptop-and-up box (a viewport clamp) where it
// renders about a pixel across the band and thickens gently toward 4K; radius, focus
// ring, icon and badge scale uniformly with the box.
const BASE: &[TailwindClass] = tw![
    "relative",
    "flex",
    "items-center",
    "justify-center",
    "size-full",
    "p-0",
    "rounded-[15cqi]",
    "border-[1.25cqi]",
    "border-warcraft-gold-border",
    "[background:linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold-dark)_55%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_55%,transparent)_100%)]",
    "cursor-pointer",
    "[transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease]",
    "focus:outline-none",
    "focus-visible:border-white",
    "focus-visible:text-white",
    "focus-visible:[box-shadow:0_0_0_3.75cqi_var(--color-warcraft-highlight),0_0_20cqi_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent)]",
];

const MOBILE: &[TailwindClass] = tw!["mobile:border-[2.8cqi]"];
const TABLET: &[TailwindClass] = tw!["tablet:border-[2.8cqi]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const ATTENTION: &[TailwindClass] = tw![
    "text-warcraft-gold",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "hover:[background:linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold-dark)_55%,transparent)_100%)]",
    "hover:[box-shadow:0_0_15cqi_color-mix(in_oklab,var(--color-warcraft-gold)_30%,transparent)]",
];

const CLEAR: &[TailwindClass] = tw![
    "border-warcraft-gold",
    "text-warcraft-gold",
    "[box-shadow:0_0_12.5cqi_color-mix(in_oklab,var(--color-warcraft-gold)_20%,transparent)]",
    "hover:[background:linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold-dark)_55%,transparent)_100%)]",
    "hover:[box-shadow:0_0_17.5cqi_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]",
];
states! {
    CollisionState, Attention => ATTENTION, Clear => CLEAR
}
