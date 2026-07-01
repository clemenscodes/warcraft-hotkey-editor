use crate::classes;

const BASE: &[&str] = &[
    "flex-[1_1_auto]",
    "min-w-0",
    "text-right",
    "text-[#f3e6c4]",
    "font-medium",
    "[font-variant-numeric:tabular-nums]",
    "group-data-[variant=hp]:text-[#4ade80]",
    "group-data-[variant=hp]:font-semibold",
    "group-data-[variant=hp]:text-[clamp(1.7rem,1.05rem+0.48vw,2.2rem)]",
    "group-data-[variant=mana]:text-[#60a5fa]",
    "group-data-[variant=mana]:font-semibold",
    "group-data-[variant=mana]:text-[clamp(1.7rem,1.05rem+0.48vw,2.2rem)]",
    "data-[zero=true]:text-[#4a5160]",
    "data-[zero=true]:font-normal",
];
const MOBILE: &[&str] = &[
    "mobile:group-data-[variant=hp]:text-[2.6rem]",
    "mobile:group-data-[variant=mana]:text-[2.6rem]",
];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
