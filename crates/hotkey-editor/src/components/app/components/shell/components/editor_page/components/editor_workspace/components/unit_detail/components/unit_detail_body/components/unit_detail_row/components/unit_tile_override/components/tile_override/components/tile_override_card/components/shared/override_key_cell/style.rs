use crate::{classes, styling::TailwindClass, tw};

// The hotkey-capture button in the override panel. A square gold-bordered key cap; it
// lights up with a static gold glow while capturing (`data-editing`) and widens for
// multi-character special tokens like Esc / Mouse4 (`data-special`). Class
// `.override-key-cell` is load-bearing (keyboard navigation).
const BASE: &[TailwindClass] = tw![
    "w-20",
    "h-20",
    "p-0",
    "flex",
    "items-center",
    "justify-center",
    "bg-warcraft-gold-dark/75",
    "border-2",
    "border-warcraft-gold",
    "rounded-[6px]",
    "text-warcraft-gold",
    "text-[2rem]",
    "leading-none",
    "uppercase",
    "text-center",
    "cursor-pointer",
    "[text-shadow:1px_1px_0_var(--color-warcraft-shadow),-1px_1px_0_var(--color-warcraft-shadow),1px_-1px_0_var(--color-warcraft-shadow),-1px_-1px_0_var(--color-warcraft-shadow)]",
    "transition-[box-shadow,border-color]",
    "duration-150",
    "hover:border-warcraft-gold",
    "hover:bg-warcraft-gold/12",
    "hover:shadow-[0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_50%,transparent)]",
    "focus:outline-none",
    "focus:shadow-[0_0_10px_color-mix(in_oklab,var(--color-warcraft-gold)_50%,transparent)]",
    "kb-focus:border-white",
    "kb-focus:text-white",
    "kb-focus:shadow-[0_0_0_3px_var(--color-warcraft-highlight),0_0_16px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent)]",
    "data-[editing=true]:bg-[linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-gold)_30%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)_100%)]",
    "data-[editing=true]:border-warcraft-gold",
    "data-[editing=true]:text-warcraft-gold",
    "data-[editing=true]:[box-shadow:0_0_18px_color-mix(in_oklab,var(--color-warcraft-gold)_85%,transparent),inset_0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_35%,transparent)]",
    "data-[special=true]:w-auto",
    "data-[special=true]:min-w-[5rem]",
    "data-[special=true]:px-[0.9rem]",
    "data-[special=true]:text-[1.5rem]",
    "data-[special=true]:normal-case",
    "data-[special=true]:tracking-normal",
    "data-[special=true]:whitespace-nowrap",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:w-[4.6rem]",
    "mobile:h-[4.6rem]",
    "mobile:min-w-[4.6rem]",
    "mobile:min-h-[4.6rem]",
    "mobile:text-[2.2rem]",
    "mobile:data-[special=true]:w-auto",
    "mobile:data-[special=true]:min-w-[4.6rem]",
    "mobile:data-[special=true]:px-[0.8rem]",
    "mobile:data-[special=true]:text-[1.5rem]",
];

const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
