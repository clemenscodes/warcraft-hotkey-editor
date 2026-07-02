use crate::classes;

// The scroll region around the collision cards. Vertical scroll on the sidebar
// with a gold scrollbar that fades in on hover of the list; horizontal snap
// scroll (the swipe carousel) on small screens with the scrollbar hidden.
const BASE: &[&str] = &[
    "flex-1",
    "flex",
    "flex-col",
    "min-h-0",
    "overflow-y-auto",
    "overflow-x-hidden",
    "pr-1",
    "[scrollbar-width:thin]",
    "[scrollbar-color:rgba(255,206,99,0)_transparent]",
    "transition-[scrollbar-color]",
    "duration-200",
    "group-hover:[scrollbar-color:rgba(255,206,99,0.45)_transparent]",
    "[&::-webkit-scrollbar]:w-[6px]",
    "[&::-webkit-scrollbar-track]:bg-transparent",
    "[&::-webkit-scrollbar-thumb]:bg-transparent",
    "[&::-webkit-scrollbar-thumb]:rounded-[3px]",
    "group-hover:[&::-webkit-scrollbar-thumb]:bg-[rgba(255,206,99,0.45)]",
    "[&::-webkit-scrollbar-thumb:hover]:bg-warcraft-gold",
];

const MOBILE: &[&str] = &[
    "mobile:overflow-x-auto",
    "mobile:overflow-y-hidden",
    "mobile:max-h-none",
    "mobile:pr-0",
    "mobile:pb-[4px]",
    "mobile:flex-none",
    "mobile:h-[clamp(96px,25vw,120px)]",
    "mobile:min-h-[clamp(96px,25vw,120px)]",
    "mobile:[-webkit-overflow-scrolling:touch]",
    "mobile:[overscroll-behavior-x:contain]",
    "mobile:[scroll-snap-type:x_mandatory]",
    "mobile:[scrollbar-width:none]",
    "mobile:[scroll-padding-inline-start:0.4rem]",
    "mobile:[&::-webkit-scrollbar]:hidden",
];

const TABLET: &[&str] = &[
    "tablet:overflow-x-auto",
    "tablet:overflow-y-hidden",
    "tablet:max-h-none",
    "tablet:pr-0",
    "tablet:pb-[4px]",
    "tablet:flex-none",
    "tablet:h-[clamp(96px,25vw,120px)]",
    "tablet:min-h-[clamp(96px,25vw,120px)]",
    "tablet:[-webkit-overflow-scrolling:touch]",
    "tablet:[overscroll-behavior-x:contain]",
    "tablet:[scroll-snap-type:x_mandatory]",
    "tablet:[scrollbar-width:none]",
    "tablet:[scroll-padding-inline-start:0.4rem]",
    "tablet:[&::-webkit-scrollbar]:hidden",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
