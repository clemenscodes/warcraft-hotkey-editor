use crate::{classes, states};

use super::state::SystemKeyPickerKeyState;

// Geometry, type, focus, and the wide-cap override — everything shared across
// states. The per-state fill/border/text/glow and the per-state hover live in the
// overlays below, so each such property is set exactly once per state.
const BASE: &[&str] = &[
    "min-w-0",
    "w-[4.7vw]",
    "h-[5.5vw]",
    "px-[0.2rem]",
    "flex",
    "items-center",
    "justify-center",
    "border",
    "rounded-[4px]",
    "[font-family:system-ui,sans-serif]",
    "text-[1.3vw]",
    "leading-none",
    "cursor-pointer",
    "whitespace-nowrap",
    "[transition:border-color_0.1s_ease,background_0.1s_ease,box-shadow_0.1s_ease]",
    "[text-shadow:1px_1px_0_#000,-1px_1px_0_#000,1px_-1px_0_#000,-1px_-1px_0_#000]",
    "focus:outline-none",
    "kb-focus:outline-none",
    "kb-focus:border-white",
    "kb-focus:text-white",
    "kb-focus:[box-shadow:0_0_0_2px_#fff,0_0_12px_rgba(255,255,255,0.5)]",
    "data-[wide=true]:w-[9.4vw]",
];
const MOBILE: &[&str] = &[
    "mobile:w-[6.5vw]",
    "mobile:h-[8vw]",
    "mobile:p-0",
    "mobile:text-[clamp(0.3rem,1.5vw,0.5rem)]",
    "mobile:data-[wide=true]:w-[13vw]",
];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }

const NORMAL: &[&str] = &[
    "[background:rgba(40,30,8,0.55)]",
    "border-[#6c5a1f]",
    "text-warcraft-gold",
    "[&:hover]:border-warcraft-gold",
    "[&:hover]:[background:rgba(255,206,99,0.12)]",
    "[&:hover]:[box-shadow:0_0_6px_rgba(255,206,99,0.5)]",
];
const CURRENT: &[&str] = &[
    "[background:linear-gradient(135deg,rgba(255,206,99,0.32)_0%,rgba(255,171,1,0.18)_100%)]",
    "border-warcraft-gold",
    "text-warcraft-gold",
    "[box-shadow:0_0_10px_rgba(255,206,99,0.55),inset_0_0_8px_rgba(255,206,99,0.2)]",
];
const CONFLICT: &[&str] = &[
    "[background:rgba(80,16,18,0.5)]",
    "border-[#6a2020]",
    "text-[#f0a8a8]",
    "[&:hover]:border-[#ff5a5a]",
    "[&:hover]:[background:rgba(120,24,28,0.55)]",
    "[&:hover]:[box-shadow:0_0_8px_rgba(255,90,90,0.5)]",
];

states! { SystemKeyPickerKeyState, Normal => NORMAL, Current => CURRENT, Conflict => CONFLICT }
