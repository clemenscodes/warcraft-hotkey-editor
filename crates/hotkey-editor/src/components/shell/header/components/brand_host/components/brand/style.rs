use crate::classes;

// Fills the host and lays the wordmark out as one cqi-scaled row: the gap between
// flourishes and title is a container-query length, so it shrinks in step with the
// title and flourishes as the host narrows. No fixed lengths — the host owns the size.
const BASE: &[&str] = &[
    "flex",
    "flex-row",
    "items-center",
    "justify-start",
    "w-full",
    "gap-[2.2cqi]",
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

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
