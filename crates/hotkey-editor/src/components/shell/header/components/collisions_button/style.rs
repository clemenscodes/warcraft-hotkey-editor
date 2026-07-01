use crate::{classes, states};

use super::state::CollisionState;

const BASE: &[&str] = &[
    "relative",
    "inline-flex",
    "items-center",
    "justify-center",
    "shrink-0",
    "p-0",
    "w-[5rem]",
    "h-[5rem]",
    "rounded-[12px]",
    "border",
    "border-[#6c5a1f]",
    "[background:linear-gradient(180deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)]",
    "cursor-pointer",
    "[transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease]",
    "focus:outline-none",
    "focus-visible:border-white",
    "focus-visible:text-white",
    "focus-visible:[box-shadow:0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)]",
];
const MOBILE: &[&str] = &[
    "mobile:w-11",
    "mobile:h-11",
    "mobile:min-w-11",
    "mobile:min-h-11",
    "mobile:rounded-[10px]",
];
const TABLET: &[&str] = &[
    "tablet:w-11",
    "tablet:h-11",
    "tablet:min-w-11",
    "tablet:min-h-11",
    "tablet:rounded-[10px]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }

const ATTENTION: &[&str] = &[
    "text-[#e8a23a]",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "hover:[background:linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)]",
    "hover:[box-shadow:0_0_12px_rgba(255,206,99,0.3)]",
];
const CLEAR: &[&str] = &[
    "border-warcraft-gold",
    "text-warcraft-gold",
    "[box-shadow:0_0_10px_rgba(255,206,99,0.2)]",
    "hover:[background:linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)]",
    "hover:[box-shadow:0_0_14px_rgba(255,206,99,0.45)]",
];

states! { CollisionState, Attention => ATTENTION, Clear => CLEAR }
