use super::state::SlotButtonState;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "relative",
    "flex",
    "flex-col",
    "items-center",
    "justify-center",
    "gap-[0.45rem]",
    "px-[0.6rem]",
    "py-[0.85rem]",
    "cursor-pointer",
    "text-center",
    "select-none",
    "border-solid",
    "border-[12px]",
    "[background:linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-bg-mid)_85%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-bg-base)_95%,transparent)_100%)]",
    "[border-image-source:var(--wc3-slot-frame)]",
    "[border-image-slice:12_fill]",
    "[border-image-repeat:stretch]",
    "[touch-action:manipulation]",
    "[transition:filter_0.15s_ease]",
    "[&:hover]:[filter:brightness(1.18)_drop-shadow(0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent))]",
    "focus:outline-none",
    "kb-focus:outline-none",
    "kb-focus:[filter:brightness(1.25)_drop-shadow(0_0_10px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent))]",
    "data-[compact=true]:border-[8px]",
    "data-[compact=true]:[border-image-slice:12]",
    "data-[compact=true]:px-[0.4rem]",
    "data-[compact=true]:py-[0.8rem]",
    "data-[compact=true]:gap-[0.4rem]",
    "data-[compact=true]:min-h-[11rem]",
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
    "after:text-[1.5rem]",
    "after:leading-[1.4]",
    "after:text-center",
    "after:[text-shadow:1px_1px_0_var(--color-warcraft-shadow)]",
    "after:[box-shadow:0_6px_22px_color-mix(in_oklab,var(--color-warcraft-shadow)_70%,transparent)]",
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
    "mobile:border-[8px]",
    "mobile:px-[0.3rem]",
    "mobile:py-[0.45rem]",
    "mobile:gap-[0.25rem]",
    "mobile:aspect-[1/0.95]",
    "mobile:min-h-[76px]",
    "mobile:data-[compact=true]:border-[6px]",
    "mobile:data-[compact=true]:px-[0.2rem]",
    "mobile:data-[compact=true]:py-[0.35rem]",
    "mobile:data-[compact=true]:gap-[0.2rem]",
    "mobile:data-[compact=true]:min-h-0",
    "mobile:data-[compact=true]:aspect-[1/1]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:border-[8px]",
    "tablet:px-[0.3rem]",
    "tablet:py-[0.45rem]",
    "tablet:gap-[0.25rem]",
    "tablet:aspect-[1/0.95]",
    "tablet:min-h-[76px]",
    "tablet:data-[compact=true]:border-[6px]",
    "tablet:data-[compact=true]:px-[0.2rem]",
    "tablet:data-[compact=true]:py-[0.35rem]",
    "tablet:data-[compact=true]:gap-[0.2rem]",
    "tablet:data-[compact=true]:min-h-0",
    "tablet:data-[compact=true]:aspect-[1/1]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const IDLE: &[TailwindClass] = tw![];
const EDITING: &[TailwindClass] = tw![
    "[filter:brightness(1.32)_drop-shadow(0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_75%,transparent))]"
];
const CONFLICT: &[TailwindClass] = tw![
    "[filter:drop-shadow(0_0_12px_color-mix(in_oklab,var(--color-warcraft-danger)_55%,transparent))]"
];
states! {
    SlotButtonState, Idle => IDLE, Editing => EDITING, Conflict => CONFLICT
}
