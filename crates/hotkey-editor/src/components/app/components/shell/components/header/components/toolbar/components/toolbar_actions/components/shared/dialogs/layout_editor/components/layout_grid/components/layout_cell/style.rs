use super::state::LayoutCellState;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "w-[clamp(7rem,9vh,12rem)]",
    "h-[clamp(7rem,9vh,12rem)]",
    "flex",
    "items-center",
    "justify-center",
    "p-0",
    "border-2",
    "rounded-[10px]",
    "font-friz-quadrata",
    "text-[clamp(3.5rem,5vh,6rem)]",
    "leading-none",
    "uppercase",
    "border-warcraft-gold",
    "text-warcraft-gold",
    "bg-[rgba(40,30,8,0.75)]",
    "[text-shadow:1px_1px_0_#000,-1px_1px_0_#000,1px_-1px_0_#000,-1px_-1px_0_#000]",
    "hover:[box-shadow:0_0_8px_rgba(255,206,99,0.5)]",
    "hover:bg-[rgba(255,206,99,0.12)]",
    "focus:outline-none",
    "kb-focus:outline-none",
    "kb-focus:border-white",
    "kb-focus:bg-[rgba(255,255,255,0.12)]",
    "kb-focus:[box-shadow:0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)]",
    "[@media(hover:none)]:[body[data-kb-modality]_&]:focus-visible:border-warcraft-gold",
    "[@media(hover:none)]:[body[data-kb-modality]_&]:focus-visible:bg-[rgba(40,30,8,0.75)]",
    "[@media(hover:none)]:[body[data-kb-modality]_&]:focus-visible:[box-shadow:none]",
    "[@media(hover:none)]:[body[data-kb-modality]_&]:focus-visible:text-warcraft-gold",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:w-[clamp(52px,18vw,72px)]",
    "mobile:h-[clamp(52px,18vw,72px)]",
    "mobile:text-[clamp(22px,7vw,34px)]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:w-[clamp(52px,18vw,72px)]",
    "tablet:h-[clamp(52px,18vw,72px)]",
    "tablet:text-[clamp(22px,7vw,34px)]",
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
    "[background:linear-gradient(135deg,rgba(255,206,99,0.3)_0%,rgba(255,171,1,0.18)_100%)]",
    "border-warcraft-gold",
    "text-warcraft-gold",
    "[box-shadow:0_0_18px_#ffce63d9,inset_0_0_14px_#ffce6359]",
];
states! {
    LayoutCellState, Idle => IDLE, Editing => EDITING
}
