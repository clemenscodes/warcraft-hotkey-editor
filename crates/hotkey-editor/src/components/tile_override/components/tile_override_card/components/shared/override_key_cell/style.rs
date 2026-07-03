use crate::classes;

// The hotkey-capture button in the override panel. A square gold-bordered key cap; it
// pulses while capturing (`data-editing`) and widens for multi-character special
// tokens like Esc / Mouse4 (`data-special`). Uses the global `pulse-editing`
// keyframe. Class `.override-key-cell` is load-bearing (keyboard navigation).
const BASE: &[&str] = &[
    "w-20",
    "h-20",
    "p-0",
    "flex",
    "items-center",
    "justify-center",
    "bg-[rgba(40,30,8,0.75)]",
    "border-2",
    "border-warcraft-gold",
    "rounded-[6px]",
    "text-warcraft-gold",
    "font-friz-quadrata",
    "text-[2rem]",
    "leading-none",
    "uppercase",
    "text-center",
    "cursor-pointer",
    "[text-shadow:1px_1px_0_#000,-1px_1px_0_#000,1px_-1px_0_#000,-1px_-1px_0_#000]",
    "transition-[box-shadow,border-color]",
    "duration-150",
    "hover:border-warcraft-gold",
    "hover:bg-[rgba(255,206,99,0.12)]",
    "hover:shadow-[0_0_8px_rgba(255,206,99,0.5)]",
    "focus:outline-none",
    "focus:shadow-[0_0_10px_rgba(255,206,99,0.5)]",
    "kb-focus:border-white",
    "kb-focus:text-white",
    "kb-focus:shadow-[0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)]",
    "data-[editing=true]:bg-[linear-gradient(135deg,rgba(255,206,99,0.3)_0%,rgba(255,171,1,0.18)_100%)]",
    "data-[editing=true]:border-warcraft-gold",
    "data-[editing=true]:text-warcraft-gold",
    "data-[editing=true]:shadow-[0_0_16px_rgba(255,206,99,0.65),inset_0_0_12px_rgba(255,206,99,0.25)]",
    "data-[editing=true]:animate-[pulse-editing_1s_ease-in-out_infinite_alternate]",
    "data-[special=true]:w-auto",
    "data-[special=true]:min-w-[5rem]",
    "data-[special=true]:px-[0.9rem]",
    "data-[special=true]:[font-family:system-ui,sans-serif]",
    "data-[special=true]:text-[1.5rem]",
    "data-[special=true]:normal-case",
    "data-[special=true]:tracking-normal",
    "data-[special=true]:whitespace-nowrap",
];

const MOBILE: &[&str] = &[
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

const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
