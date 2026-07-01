use crate::{classes, states};

use super::state::SlotButtonState;

// The gold nine-slice slot. The frame image arrives from an ancestor as the
// `--wc3-slot-frame` variable. Compact (control-group) cells shrink the frame and
// padding via `data-compact`; the small-viewport sizing lives in the bands. The
// per-state glow is layered in the overlays below.
const BASE: &[&str] = &[
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
    "[background:linear-gradient(180deg,rgba(15,22,45,0.85)_0%,rgba(8,14,30,0.95)_100%)]",
    "[border-image-source:var(--wc3-slot-frame)]",
    "[border-image-slice:12_fill]",
    "[border-image-repeat:stretch]",
    "[touch-action:manipulation]",
    "[transition:filter_0.15s_ease]",
    "[&:hover]:[filter:brightness(1.18)_drop-shadow(0_0_8px_rgba(255,206,99,0.45))]",
    "focus:outline-none",
    "kb-focus:outline-none",
    "kb-focus:[filter:brightness(1.25)_drop-shadow(0_0_10px_rgba(255,255,255,0.55))]",
    "data-[compact=true]:border-[8px]",
    "data-[compact=true]:[border-image-slice:12]",
    "data-[compact=true]:px-[0.4rem]",
    "data-[compact=true]:py-[0.8rem]",
    "data-[compact=true]:gap-[0.4rem]",
    "data-[compact=true]:min-h-[11rem]",
];
const MOBILE: &[&str] = &[
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
const TABLET: &[&str] = &[
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
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }

const IDLE: &[&str] = &[];
const EDITING: &[&str] = &["[filter:brightness(1.32)_drop-shadow(0_0_14px_rgba(255,206,99,0.75))]"];
const CONFLICT: &[&str] = &["[filter:drop-shadow(0_0_12px_rgba(255,90,90,0.55))]"];

states! { SlotButtonState, Idle => IDLE, Editing => EDITING, Conflict => CONFLICT }
