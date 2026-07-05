use super::state::SystemHotkeysCategoryTabState;
use crate::{classes, states, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "uppercase",
    "tracking-[0.1em]",
    "text-[2rem]",
    "leading-none",
    "px-3",
    "py-1",
    "m-0",
    "bg-transparent",
    "border-0",
    "cursor-pointer",
    "whitespace-nowrap",
    "text-shadow-drop",
    "[transition:color_0.15s_ease,text-shadow_0.15s_ease]",
    "hover:text-warcraft-gold",
    "hover:text-shadow-glow-12",
    "kb-focus:outline-none",
    "kb-focus:text-white",
    "kb-focus:[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_14px_color-mix(in_oklab,var(--color-warcraft-highlight)_65%,transparent)]",
    "group-data-[open=true]:flex-none",
    "group-data-[open=true]:w-full",
    "group-data-[open=true]:text-left",
    "group-data-[open=true]:py-[0.7rem]",
    "group-data-[open=true]:px-[0.85rem]",
    "group-data-[open=true]:text-[clamp(14px,3.8vw,17px)]",
    "group-data-[open=true]:tracking-[0.05em]",
    "group-data-[open=true]:min-h-[44px]",
    "group-data-[open=true]:rounded-[6px]",
    "group-data-[open=true]:whitespace-normal",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

const ACTIVE: &[TailwindClass] = tw![
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_16px_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]",
    "group-data-[open=true]:bg-warcraft-gold/14",
    "group-data-[open=true]:[box-shadow:inset_0_0_0_1px_color-mix(in_oklab,var(--color-warcraft-gold)_40%,transparent)]",
];

const INACTIVE: &[TailwindClass] = tw!["text-warcraft-gold/55"];
states! {
    SystemHotkeysCategoryTabState, Active => ACTIVE, Inactive => INACTIVE
}
