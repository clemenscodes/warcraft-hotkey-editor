use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "group",
    "flex",
    "items-baseline",
    "justify-center",
    "flex-wrap",
    "gap-3",
    "[flex:1_1_auto]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:hidden",
    "mobile:data-[open=true]:flex",
    "mobile:data-[open=true]:absolute",
    "mobile:data-[open=true]:top-[calc(100%-0.25rem)]",
    "mobile:data-[open=true]:left-3",
    "mobile:data-[open=true]:right-3",
    "mobile:data-[open=true]:z-[6]",
    "mobile:data-[open=true]:flex-col",
    "mobile:data-[open=true]:items-stretch",
    "mobile:data-[open=true]:gap-[0.15rem]",
    "mobile:data-[open=true]:p-[0.4rem]",
    "mobile:data-[open=true]:[background:linear-gradient(170deg,#0c1d30_0%,#070e1c_100%)]",
    "mobile:data-[open=true]:border",
    "mobile:data-[open=true]:border-[rgba(255,206,99,0.45)]",
    "mobile:data-[open=true]:rounded-[10px]",
    "mobile:data-[open=true]:[box-shadow:0_14px_30px_rgba(0,0,0,0.7),0_0_18px_rgba(255,206,99,0.12)]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:hidden",
    "tablet:data-[open=true]:flex",
    "tablet:data-[open=true]:absolute",
    "tablet:data-[open=true]:top-[calc(100%-0.25rem)]",
    "tablet:data-[open=true]:left-3",
    "tablet:data-[open=true]:right-3",
    "tablet:data-[open=true]:z-[6]",
    "tablet:data-[open=true]:flex-col",
    "tablet:data-[open=true]:items-stretch",
    "tablet:data-[open=true]:gap-[0.15rem]",
    "tablet:data-[open=true]:p-[0.4rem]",
    "tablet:data-[open=true]:[background:linear-gradient(170deg,#0c1d30_0%,#070e1c_100%)]",
    "tablet:data-[open=true]:border",
    "tablet:data-[open=true]:border-[rgba(255,206,99,0.45)]",
    "tablet:data-[open=true]:rounded-[10px]",
    "tablet:data-[open=true]:[box-shadow:0_14px_30px_rgba(0,0,0,0.7),0_0_18px_rgba(255,206,99,0.12)]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
