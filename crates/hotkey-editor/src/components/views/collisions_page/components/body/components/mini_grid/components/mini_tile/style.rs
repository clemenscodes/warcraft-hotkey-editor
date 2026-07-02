use crate::classes;

const BASE: &[&str] = &[
    "bg-[rgba(13,31,61,0.65)]",
    "border",
    "border-[#24406a]",
    "rounded-[2px]",
    "data-[highlighted=true]:[border-color:var(--race-color,#ffce63)]",
    "data-[highlighted=true]:bg-[rgba(255,206,99,0.2)]",
    "data-[highlighted=true]:[box-shadow:0_0_6px_var(--race-color-soft,rgba(255,206,99,0.5))]",
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
