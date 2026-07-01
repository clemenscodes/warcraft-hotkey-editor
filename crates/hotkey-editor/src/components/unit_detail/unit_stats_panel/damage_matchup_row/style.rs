use crate::classes;

const BASE: &[&str] = &[
    "grid",
    "grid-cols-[repeat(2,minmax(0,1fr))]",
    "gap-x-[0.5rem]",
    "gap-y-[0.25rem]",
    "mt-auto",
    "pt-[0.75rem]",
    "border-t",
    "border-t-[rgba(255,206,99,0.15)]",
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
