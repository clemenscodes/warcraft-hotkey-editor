use crate::{classes, states};

use super::state::SystemHotkeysCategoryTabState;

// A single category label. On desktop it is an inline gold tab; inside the open
// mobile popover (`group-data-[open=true]`) it stretches to a full-width menu row.
const BASE: &[&str] = &[
    "font-friz-quadrata",
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
    "[text-shadow:1px_1px_0_#000]",
    "[transition:color_0.15s_ease,text-shadow_0.15s_ease]",
    "hover:text-warcraft-gold",
    "hover:[text-shadow:1px_1px_0_#000,0_0_12px_rgba(255,206,99,0.55)]",
    "kb-focus:outline-none",
    "kb-focus:text-white",
    "kb-focus:[text-shadow:1px_1px_0_#000,0_0_14px_rgba(255,255,255,0.65)]",
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
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }

const ACTIVE: &[&str] = &[
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_#000,0_0_16px_rgba(255,206,99,0.45)]",
    "group-data-[open=true]:bg-[rgba(255,206,99,0.14)]",
    "group-data-[open=true]:[box-shadow:inset_0_0_0_1px_rgba(255,206,99,0.4)]",
];
const INACTIVE: &[&str] = &["text-warcraft-gold/55"];

states! { SystemHotkeysCategoryTabState, Active => ACTIVE, Inactive => INACTIVE }
