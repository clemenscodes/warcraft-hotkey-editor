use super::state::KeyCaptureCellState;
use crate::{classes, states};

const BASE: &[&str] = &[
    "inline-flex",
    "items-center",
    "justify-center",
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.04em]",
    "text-[2.4rem]",
    "leading-none",
    "px-6",
    "py-3.5",
    "min-w-[18rem]",
    "cursor-pointer",
    "whitespace-nowrap",
    "border",
    "rounded-[2px]",
    "[background:linear-gradient(180deg,rgba(15,22,45,0.85)_0%,rgba(8,14,30,0.95)_100%)]",
    "[transition:filter_0.15s_ease,border-color_0.15s_ease]",
    "hover:[filter:brightness(1.18)_drop-shadow(0_0_8px_rgba(255,206,99,0.4))]",
    "hover:border-[rgba(255,206,99,0.85)]",
    "kb-focus:outline-none",
    "kb-focus:border-white",
    "kb-focus:[filter:drop-shadow(0_0_10px_rgba(255,255,255,0.55))]",
    "relative",
    "after:[content:attr(data-tooltip)]",
    "after:absolute",
    "after:bottom-[calc(100%+0.6rem)]",
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
    "[&[data-tooltip]:not([data-tooltip=''])]:hover:after:opacity-100",
    "[&[data-tooltip]:not([data-tooltip=''])]:focus-visible:after:opacity-100",
];

const MOBILE: &[&str] = &[
    "mobile:min-w-[5.5rem]",
    "mobile:max-w-[14rem]",
    "mobile:px-[0.7rem]",
    "mobile:py-[0.5rem]",
    "mobile:text-[clamp(13px,3.6vw,16px)]",
    "mobile:[flex:0_0_auto]",
    "mobile:overflow-hidden",
    "mobile:text-ellipsis",
    "mobile:[touch-action:manipulation]",
];

const TABLET: &[&str] = &[
    "tablet:min-w-[5.5rem]",
    "tablet:max-w-[14rem]",
    "tablet:px-[0.7rem]",
    "tablet:py-[0.5rem]",
    "tablet:text-[clamp(13px,3.6vw,16px)]",
    "tablet:[flex:0_0_auto]",
    "tablet:overflow-hidden",
    "tablet:text-ellipsis",
    "tablet:[touch-action:manipulation]",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const NORMAL: &[&str] = &[
    "text-warcraft-gold",
    "border-[rgba(255,206,99,0.45)]",
    "[text-shadow:1px_1px_0_#000]",
];

const CONFLICT: &[&str] = &[
    "text-[#ff5a5a]",
    "border-[rgba(255,90,90,0.65)]",
    "[text-shadow:1px_1px_0_#000,0_0_10px_rgba(255,90,90,0.5)]",
];
states! {
    KeyCaptureCellState, Normal => NORMAL, Conflict => CONFLICT
}
