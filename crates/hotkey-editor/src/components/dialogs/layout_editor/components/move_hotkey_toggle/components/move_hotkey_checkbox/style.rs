use crate::classes;

const BASE: &[&str] = &[
    "appearance-none",
    "inline-flex",
    "items-center",
    "justify-center",
    "flex-none",
    "w-[2rem]",
    "h-[2rem]",
    "m-0",
    "border-2",
    "rounded-[8px]",
    "border-warcraft-gold",
    "bg-[rgba(40,30,8,0.75)]",
    "cursor-pointer",
    "[transition:background_0.15s_ease,box-shadow_0.15s_ease]",
    "hover:bg-[rgba(255,206,99,0.12)]",
    "hover:[box-shadow:0_0_8px_rgba(255,206,99,0.5)]",
    "checked:bg-[rgba(255,206,99,0.18)]",
    "checked:[box-shadow:inset_0_0_8px_rgba(255,206,99,0.25)]",
    "checked:after:content-['']",
    "checked:after:w-[0.6rem]",
    "checked:after:h-[1.05rem]",
    "checked:after:mt-[-0.18rem]",
    "checked:after:border-solid",
    "checked:after:border-warcraft-gold",
    "checked:after:[border-width:0_0.3rem_0.3rem_0]",
    "checked:after:rotate-45",
    "checked:after:[filter:drop-shadow(1px_1px_0_#000)]",
    "focus-visible:outline-none",
    "focus-visible:[box-shadow:0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)]",
];

const MOBILE: &[&str] = &[
    "mobile:w-[22px]",
    "mobile:h-[22px]",
    "mobile:rounded-[6px]",
    "mobile:checked:after:w-[6px]",
    "mobile:checked:after:h-[11px]",
    "mobile:checked:after:mt-[-2px]",
    "mobile:checked:after:[border-width:0_3px_3px_0]",
];

const TABLET: &[&str] = &[
    "tablet:w-[22px]",
    "tablet:h-[22px]",
    "tablet:rounded-[6px]",
    "tablet:checked:after:w-[6px]",
    "tablet:checked:after:h-[11px]",
    "tablet:checked:after:mt-[-2px]",
    "tablet:checked:after:[border-width:0_3px_3px_0]",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
