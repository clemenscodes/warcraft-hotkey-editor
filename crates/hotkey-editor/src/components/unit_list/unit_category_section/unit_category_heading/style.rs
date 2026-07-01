use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "items-center",
    "gap-[0.4rem]",
    "w-full",
    "text-left",
    "mt-3",
    "mb-1",
    "py-[0.35rem]",
    "px-[0.25rem]",
    "font-friz-quadrata",
    "text-[1.2rem]",
    "uppercase",
    "tracking-[0.12em]",
    "border-b",
    "border-[#1f3d63]",
    "cursor-pointer",
    "transition-colors",
    "duration-[0.12s]",
    "first:mt-0",
    "text-[#7b818d]",
    "data-[collapsed=true]:text-[#5a6075]",
    "hover:text-warcraft-gold",
    "focus:outline-none",
    "kb-focus:text-warcraft-gold",
];

const MOBILE: &[&str] = &["mobile:hidden"];
const TABLET: &[&str] = &["tablet:hidden"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
