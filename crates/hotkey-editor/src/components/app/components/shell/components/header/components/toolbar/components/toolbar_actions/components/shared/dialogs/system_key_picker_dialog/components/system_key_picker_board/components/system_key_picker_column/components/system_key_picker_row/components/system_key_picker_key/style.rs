use super::state::SystemKeyPickerKeyState;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "relative",
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
    "after:[content:attr(data-tooltip)]",
    "after:absolute",
    "after:top-[calc(100%+0.6rem)]",
    "after:left-1/2",
    "after:-translate-x-1/2",
    "after:w-max",
    "after:max-w-[38rem]",
    "after:px-[1.1rem]",
    "after:py-3",
    "after:border",
    "after:border-warcraft-gold",
    "after:rounded-[4px]",
    "after:[background:linear-gradient(180deg,#0f162dfa_0%,#080e1efc_100%)]",
    "after:text-warcraft-gold",
    "after:font-friz-quadrata",
    "after:text-[1.5rem]",
    "after:leading-[1.4]",
    "after:text-center",
    "after:[text-shadow:1px_1px_0_#000]",
    "after:[box-shadow:0_6px_22px_#000000b3]",
    "after:whitespace-normal",
    "after:pointer-events-none",
    "after:z-[1200]",
    "after:opacity-0",
    "after:transition-opacity",
    "after:duration-150",
    "after:delay-[400ms]",
    "data-[tooltip-placement=above]:after:top-auto",
    "data-[tooltip-placement=above]:after:bottom-[calc(100%+0.6rem)]",
    "data-[tooltip-anchor=left]:after:left-0",
    "data-[tooltip-anchor=left]:after:right-auto",
    "data-[tooltip-anchor=left]:after:translate-x-0",
    "data-[tooltip-anchor=right]:after:left-auto",
    "data-[tooltip-anchor=right]:after:right-0",
    "data-[tooltip-anchor=right]:after:translate-x-0",
    "[&[data-tooltip]:not([data-tooltip=''])]:hover:after:opacity-100",
    "[&[data-tooltip]:not([data-tooltip=''])]:focus-visible:after:opacity-100",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:w-[6.5vw]",
    "mobile:h-[8vw]",
    "mobile:p-0",
    "mobile:text-[clamp(0.3rem,1.5vw,0.5rem)]",
    "mobile:data-[wide=true]:w-[13vw]",
];

const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const NORMAL: &[TailwindClass] = tw![
    "[background:rgba(40,30,8,0.55)]",
    "border-[#6c5a1f]",
    "text-warcraft-gold",
    "[&:hover]:border-warcraft-gold",
    "[&:hover]:[background:rgba(255,206,99,0.12)]",
    "[&:hover]:[box-shadow:0_0_6px_rgba(255,206,99,0.5)]",
];

const CURRENT: &[TailwindClass] = tw![
    "[background:linear-gradient(135deg,rgba(255,206,99,0.32)_0%,rgba(255,171,1,0.18)_100%)]",
    "border-warcraft-gold",
    "text-warcraft-gold",
    "[box-shadow:0_0_10px_rgba(255,206,99,0.55),inset_0_0_8px_rgba(255,206,99,0.2)]",
];

const CONFLICT: &[TailwindClass] = tw![
    "[background:rgba(80,16,18,0.5)]",
    "border-[#6a2020]",
    "text-[#f0a8a8]",
    "[&:hover]:border-[#ff5a5a]",
    "[&:hover]:[background:rgba(120,24,28,0.55)]",
    "[&:hover]:[box-shadow:0_0_8px_rgba(255,90,90,0.5)]",
];
states! {
    SystemKeyPickerKeyState, Normal => NORMAL, Current => CURRENT, Conflict => CONFLICT
}
