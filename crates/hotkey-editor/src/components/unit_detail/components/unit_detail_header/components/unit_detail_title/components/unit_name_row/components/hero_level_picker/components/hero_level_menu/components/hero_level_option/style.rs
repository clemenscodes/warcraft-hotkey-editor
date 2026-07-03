use crate::classes;

const BASE: &[&str] = &[
    "block",
    "w-full",
    "py-[0.4rem]",
    "px-[0.7rem]",
    "bg-transparent",
    "border-none",
    "rounded-[5px]",
    "text-warcraft-text-secondary",
    "font-mono",
    "text-[1.4rem]/[1.25]",
    "text-left",
    "cursor-pointer",
    "whitespace-nowrap",
    "[transition:background_0.1s_ease,color_0.1s_ease]",
    "hover:bg-[rgba(255,206,99,0.12)]",
    "hover:text-warcraft-gold",
    "data-[active=true]:[background:linear-gradient(135deg,rgba(255,206,99,0.25)_0%,rgba(60,45,14,0.7)_100%)]",
    "data-[active=true]:text-warcraft-gold",
    "[body[data-kb-modality]_&]:focus:outline-none",
    "[body[data-kb-modality]_&]:focus:text-white",
    "[body[data-kb-modality]_&]:focus:[box-shadow:inset_0_0_0_2px_#fff]",
];
const MOBILE: &[&str] = &[
    "mobile:px-[10px]",
    "mobile:py-[3px]",
    "mobile:min-h-[24px]",
    "mobile:text-[16px]",
];
const TABLET: &[&str] = &[
    "tablet:px-[10px]",
    "tablet:py-[3px]",
    "tablet:min-h-[24px]",
    "tablet:text-[13px]",
];
const LAPTOP: &[&str] = &[
    "laptop:px-[0.55rem]",
    "laptop:py-[0.3rem]",
    "laptop:text-[1.05rem]",
];
const DESKTOP: &[&str] = &[
    "desktop:px-[0.55rem]",
    "desktop:py-[0.3rem]",
    "desktop:text-[1.05rem]",
];
const QHD: &[&str] = &["qhd:px-[0.55rem]", "qhd:py-[0.3rem]", "qhd:text-[1.05rem]"];
const UHD: &[&str] = &["uhd:px-[0.55rem]", "uhd:py-[0.3rem]", "uhd:text-[1.05rem]"];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
