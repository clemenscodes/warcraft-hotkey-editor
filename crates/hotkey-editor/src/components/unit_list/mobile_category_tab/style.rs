use crate::classes;

// One category pill in the mobile tab row (shown only on small screens, where the
// container's nav switches to flex). Active takes the inherited race color.
const BASE: &[&str] = &[
    "flex-1",
    "min-w-0",
    "min-h-[44px]",
    "px-[0.5rem]",
    "bg-[rgba(13,31,61,0.55)]",
    "border",
    "border-[#1f3d63]",
    "rounded-[8px]",
    "text-[#c0c8d4]",
    "font-friz-quadrata",
    "text-[0.95rem]",
    "tracking-[0.04em]",
    "uppercase",
    "text-center",
    "cursor-pointer",
    "transition-all",
    "duration-[0.12s]",
    "whitespace-nowrap",
    "overflow-hidden",
    "text-ellipsis",
    "hover:bg-[rgba(30,60,95,0.7)]",
    "hover:text-white",
    "hover:border-[color:var(--race-color,#2a5085)]",
    "focus:outline-none",
    "kb-focus:border-white",
    "kb-focus:shadow-[0_0_0_2px_#fff]",
    "data-[active=true]:bg-[linear-gradient(135deg,rgba(45,80,130,0.95)_0%,rgba(20,45,80,0.95)_100%)]",
    "data-[active=true]:border-[color:var(--race-color,#ffce63)]",
    "data-[active=true]:text-[color:var(--race-color,#ffce63)]",
    "data-[active=true]:shadow-[0_0_6px_var(--race-color-soft,rgba(255,206,99,0.3))]",
];
const MOBILE: &[&str] = &[
    "mobile:text-[clamp(11px,2.8vw,14px)]",
    "mobile:px-[0.35rem]",
    "mobile:h-[44px]",
    "mobile:leading-none",
];
const TABLET: &[&str] = &[
    "tablet:text-[clamp(11px,2.8vw,14px)]",
    "tablet:px-[0.35rem]",
    "tablet:h-[44px]",
    "tablet:leading-none",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
