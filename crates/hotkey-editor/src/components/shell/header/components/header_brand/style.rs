use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-row",
    "items-center",
    "justify-start",
    "min-w-0",
    "flex-initial",
    "gap-4",
    "bg-transparent",
    "border-0",
    "p-0",
    "cursor-pointer",
    "text-left",
    "[transition:filter_0.12s_ease,text-shadow_0.12s_ease]",
    "hover:[filter:brightness(1.15)]",
    "focus:outline-none",
    "focus-visible:[outline:2px_solid_#fff]",
    "focus-visible:[outline-offset:2px]",
];

const MOBILE: &[&str] = &["mobile:gap-2", "mobile:flex-auto"];
const TABLET: &[&str] = &["tablet:gap-2", "tablet:flex-auto"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
