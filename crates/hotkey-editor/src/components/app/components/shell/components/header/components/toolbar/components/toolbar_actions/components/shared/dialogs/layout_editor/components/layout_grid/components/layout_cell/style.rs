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
    "[text-shadow:1px_1px_0_var(--color-warcraft-shadow),-1px_1px_0_var(--color-warcraft-shadow),1px_-1px_0_var(--color-warcraft-shadow),-1px_-1px_0_var(--color-warcraft-shadow)]",
    "hover:[box-shadow:0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_50%,transparent)]",
    "hover:bg-warcraft-gold/12",
    "focus:outline-none",
    "kb-focus:outline-none",
    "kb-focus:border-white",
    "kb-focus:bg-warcraft-highlight/12",
    "kb-focus:[box-shadow:0_0_0_3px_var(--color-warcraft-highlight),0_0_16px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent)]",
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
    "[background:linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-gold)_30%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)_100%)]",
    "border-warcraft-gold",
    "text-warcraft-gold",
    "[box-shadow:0_0_18px_color-mix(in_oklab,var(--color-warcraft-gold)_85%,transparent),inset_0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_35%,transparent)]",
];
states! {
    LayoutCellState, Idle => IDLE, Editing => EDITING
}
