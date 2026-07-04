use super::state::CollisionState;
use crate::{classes, states};

// The button surface fills its host container and draws itself as a cqi-scaled drawing:
// every interior length resolves against the host box (the host is the query context).
// The box is a fixed size per band, so the border is tuned per band to land on 1px at
// each size (1.25cqi at the 80px desktop box, 2.8cqi at the 36px compact box); radius,
// focus ring, icon and badge scale uniformly.
const BASE: &[&str] = &[
    "relative",
    "flex",
    "items-center",
    "justify-center",
    "size-full",
    "p-0",
    "rounded-[15cqi]",
    "border-[1.25cqi]",
    "border-[#6c5a1f]",
    "[background:linear-gradient(180deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)]",
    "cursor-pointer",
    "[transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease]",
    "focus:outline-none",
    "focus-visible:border-white",
    "focus-visible:text-white",
    "focus-visible:[box-shadow:0_0_0_3.75cqi_#fff,0_0_20cqi_rgba(255,255,255,0.55)]",
];

const MOBILE: &[&str] = &["mobile:border-[2.8cqi]"];
const TABLET: &[&str] = &["tablet:border-[2.8cqi]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const ATTENTION: &[&str] = &[
    "text-[#e8a23a]",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "hover:[background:linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)]",
    "hover:[box-shadow:0_0_15cqi_rgba(255,206,99,0.3)]",
];

const CLEAR: &[&str] = &[
    "border-warcraft-gold",
    "text-warcraft-gold",
    "[box-shadow:0_0_12.5cqi_rgba(255,206,99,0.2)]",
    "hover:[background:linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)]",
    "hover:[box-shadow:0_0_17.5cqi_rgba(255,206,99,0.45)]",
];
states! {
    CollisionState, Attention => ATTENTION, Clear => CLEAR
}
