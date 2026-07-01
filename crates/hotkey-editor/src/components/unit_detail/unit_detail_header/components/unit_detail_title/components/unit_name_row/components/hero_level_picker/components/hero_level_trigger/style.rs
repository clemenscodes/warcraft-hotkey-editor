use crate::classes;

const BASE: &[&str] = &[
    "group",
    "flex",
    "items-center",
    "justify-between",
    "gap-[0.5rem]",
    "w-[8.25rem]",
    "px-[0.5rem]",
    "py-[0.25rem]",
    "whitespace-nowrap",
    "[background:linear-gradient(135deg,rgba(40,30,8,0.6)_0%,rgba(15,12,4,0.6)_100%)]",
    "border",
    "border-[rgba(255,206,99,0.45)]",
    "rounded-[6px]",
    "text-warcraft-gold",
    "font-friz-quadrata",
    "text-[1.15rem]",
    "uppercase",
    "tracking-[0.06em]",
    "[text-shadow:1px_1px_0_#000]",
    "cursor-pointer",
    "[transition:border-color_0.15s_ease,box-shadow_0.15s_ease]",
    "hover:border-warcraft-gold",
    "hover:[box-shadow:0_0_8px_rgba(255,206,99,0.3)]",
    "data-[open=true]:border-warcraft-gold",
    "data-[open=true]:[box-shadow:0_0_12px_rgba(255,206,99,0.4)]",
    "[body[data-kb-modality]_&]:focus:outline-none",
    "[body[data-kb-modality]_&]:focus:border-white",
    "[body[data-kb-modality]_&]:focus:text-white",
    "[body[data-kb-modality]_&]:focus:[box-shadow:0_0_0_2px_#fff,0_0_14px_rgba(255,255,255,0.45)]",
];
const MOBILE: &[&str] = &["mobile:w-[10.5rem]", "mobile:px-[0.65rem]", "mobile:py-[0.3rem]", "mobile:text-[1.55rem]"];
const TABLET: &[&str] = &["tablet:w-[10.5rem]", "tablet:px-[0.65rem]", "tablet:py-[0.3rem]", "tablet:text-[1.55rem]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
