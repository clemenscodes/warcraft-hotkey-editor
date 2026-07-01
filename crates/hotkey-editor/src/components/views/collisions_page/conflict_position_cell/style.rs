use crate::classes;

const BASE: &[&str] = &[
    "self-start",
    "h-[72px]",
    "inline-flex",
    "items-center",
    "justify-center",
    "[&>*]:h-[60px]",
    "[&>*]:w-[calc(60px/3*4)]",
    "data-[top=true]:self-center",
    "data-[top=true]:h-auto",
    "data-[top=true]:mb-[10px]",
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
