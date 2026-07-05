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
    "text-[clamp(3.5rem,5vh,6rem)]",
    "leading-none",
    "uppercase",
    "border-warcraft-gold",
    "text-warcraft-gold",
    "bg-warcraft-gold-dark/75",
    "text-shadow-outline",
    "hover:shadow-glow-8",
    "hover:bg-warcraft-gold/12",
    "focus:outline-none",
    "kb-focus:outline-none",
    "kb-focus:border-white",
    "kb-focus:bg-warcraft-highlight/12",
    "kb-focus:focus-ring",
    "[@media(hover:none)]:[body[data-kb-modality]_&]:focus-visible:border-warcraft-gold",
    "[@media(hover:none)]:[body[data-kb-modality]_&]:focus-visible:bg-warcraft-gold-dark/75",
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
    "bg-panel-gold-diag-30",
    "border-warcraft-gold",
    "text-warcraft-gold",
    "shadow-glow-18",
];
states! {
    LayoutCellState, Idle => IDLE, Editing => EDITING
}
