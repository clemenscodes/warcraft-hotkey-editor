use super::state::KeyCaptureCellState;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "inline-flex",
    "items-center",
    "justify-center",
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
    "bg-panel-dark-85",
    "[transition:filter_0.15s_ease,border-color_0.15s_ease]",
    "hover:[filter:brightness(1.18)_drop-shadow(0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_40%,transparent))]",
    "hover:border-warcraft-gold/85",
    "kb-focus:outline-none",
    "kb-focus:border-white",
    "kb-focus:[filter:drop-shadow(0_0_10px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent))]",
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
    "after:bg-panel-dark-solid",
    "after:text-warcraft-gold",
    "after:text-[1.5rem]",
    "after:leading-[1.4]",
    "after:text-center",
    "after:text-shadow-drop",
    "after:shadow-elevation",
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

const MOBILE: &[TailwindClass] = tw![
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

const TABLET: &[TailwindClass] = tw![
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

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const NORMAL: &[TailwindClass] = tw![
    "text-warcraft-gold",
    "border-warcraft-gold/45",
    "text-shadow-drop",
];

const CONFLICT: &[TailwindClass] = tw![
    "text-warcraft-danger",
    "border-warcraft-danger/65",
    "[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_10px_color-mix(in_oklab,var(--color-warcraft-danger)_50%,transparent)]",
];
states! {
    KeyCaptureCellState, Normal => NORMAL, Conflict => CONFLICT
}
