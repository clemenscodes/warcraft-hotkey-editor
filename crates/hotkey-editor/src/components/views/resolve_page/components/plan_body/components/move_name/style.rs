use crate::classes;
const BASE: &[&str] = &[
    "font-friz-quadrata",
    "text-[1.7rem]",
    "text-warcraft-gold",
    "whitespace-nowrap",
    "min-w-0",
    "data-[link=true]:cursor-pointer",
    "group-[:not(:disabled):hover]:data-[link=true]:text-white",
    "group-[:not(:disabled):hover]:data-[link=true]:underline",
    "group-[:not(:disabled):hover]:data-[link=true]:[text-underline-offset:2px]",
];
const MOBILE: &[&str] = &["mobile:text-[max(0.6rem,min(1.7rem,calc((100vw_-_88px)/27.5)))]"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
