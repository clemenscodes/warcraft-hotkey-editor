use crate::classes;

// The unit portrait thumbnail. 80px on the sidebar; a larger fluid square in the
// mobile carousel tile; a touch smaller on mid laptops. Empty div fallback shares
// the frame. `text-[0]` hides the alt text if the image fails.
const BASE: &[&str] = &[
    "w-20",
    "h-20",
    "shrink-0",
    "object-cover",
    "border",
    "border-[#2a5085]",
    "rounded-[3px]",
    "bg-[rgba(20,35,60,0.7)]",
    "text-transparent",
    "text-[0]",
    "leading-[0]",
    "[image-rendering:auto]",
];
const MOBILE: &[&str] = &[
    "mobile:w-[clamp(62px,16vw,78px)]",
    "mobile:h-[clamp(62px,16vw,78px)]",
];
const TABLET: &[&str] = &[
    "tablet:w-[clamp(62px,16vw,78px)]",
    "tablet:h-[clamp(62px,16vw,78px)]",
];
const LAPTOP: &[&str] = &[
    "laptop:w-[clamp(40px,4vw+16px,64px)]",
    "laptop:h-[clamp(40px,4vw+16px,64px)]",
];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
