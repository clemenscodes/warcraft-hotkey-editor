use super::state::KeyPickerKeyState;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "w-[clamp(5rem,7.5vw,11rem)]",
    "h-[clamp(5rem,7.5vw,11rem)]",
    "flex",
    "flex-col",
    "items-center",
    "justify-center",
    "gap-[0.2rem]",
    "p-0",
    "border",
    "rounded-[6px]",
    "text-[clamp(2rem,3.5vw,5rem)]",
    "leading-none",
    "cursor-pointer",
    "[transition:border-color_0.12s_ease,background_0.12s_ease,box-shadow_0.12s_ease]",
    "[text-shadow:1px_1px_0_#000,-1px_1px_0_#000,1px_-1px_0_#000,-1px_-1px_0_#000]",
    "[&:hover:not(:disabled)]:border-warcraft-gold",
    "[&:hover:not(:disabled)]:[background:rgba(255,206,99,0.12)]",
    "[&:hover:not(:disabled)]:[box-shadow:0_0_8px_rgba(255,206,99,0.5)]",
    "focus:outline-none",
    "kb-focus:outline-none",
    "kb-focus:border-white",
    "kb-focus:text-white",
    "kb-focus:[box-shadow:0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)]",
    "data-[special=true]:w-auto",
    "data-[special=true]:min-w-[clamp(5rem,7.5vw,11rem)]",
    "data-[special=true]:px-[0.85rem]",
    "data-[special=true]:text-[clamp(1rem,1.6vw,2rem)]",
    "data-[special=true]:whitespace-nowrap",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:w-[clamp(2.5rem,8.5vw,5rem)]",
    "mobile:h-[clamp(2.5rem,8.5vw,5rem)]",
    "mobile:text-[clamp(1.1rem,3.5vw,2.2rem)]",
    "mobile:data-[special=true]:min-w-[clamp(2.5rem,8.5vw,5rem)]",
    "mobile:data-[special=true]:px-[0.55rem]",
    "mobile:data-[special=true]:text-[clamp(0.75rem,2.6vw,1.25rem)]",
];

const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const AVAILABLE: &[TailwindClass] = tw![
    "[background:rgba(40,30,8,0.55)]",
    "border-[#6c5a1f]",
    "text-warcraft-gold",
];

const CURRENT: &[TailwindClass] = tw![
    "[background:linear-gradient(135deg,rgba(255,206,99,0.32)_0%,rgba(255,171,1,0.18)_100%)]",
    "border-warcraft-gold",
    "text-warcraft-gold",
    "[box-shadow:0_0_14px_rgba(255,206,99,0.55),inset_0_0_10px_rgba(255,206,99,0.22)]",
];

const CONFLICT: &[TailwindClass] = tw![
    "[background:rgba(80,16,18,0.5)]",
    "border-[#6a2020]",
    "text-[#f0a8a8]",
    "cursor-not-allowed",
    "opacity-85",
];
states! {
    KeyPickerKeyState, Available => AVAILABLE, Current => CURRENT, Conflict => CONFLICT
}
