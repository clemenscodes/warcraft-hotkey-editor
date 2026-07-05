use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "items-center",
    "gap-[0.4rem]",
    "w-full",
    "text-left",
    "mt-3",
    "mb-1",
    "py-[0.35rem]",
    "px-[0.25rem]",
    "text-[1.2rem]",
    "uppercase",
    "tracking-[0.12em]",
    "border-b",
    "border-warcraft-blue-deep",
    "cursor-pointer",
    "transition-colors",
    "duration-[0.12s]",
    "first:mt-0",
    "text-warcraft-text-faint",
    "data-[collapsed=true]:text-warcraft-text-faint",
    "hover:text-warcraft-gold",
    "focus:outline-none",
    "kb-focus:text-warcraft-gold",
];

const MOBILE: &[TailwindClass] = tw!["mobile:hidden"];
const TABLET: &[TailwindClass] = tw!["tablet:hidden"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
