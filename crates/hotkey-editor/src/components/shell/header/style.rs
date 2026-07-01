use crate::classes;

const BASE: &[&str] = &[
    "relative",
    "z-50",
    "items-center",
    "flex-none",
    "border-b",
    "border-b-[rgba(255,206,99,0.4)]",
    "[box-shadow:0_1px_0_rgba(0,0,0,0.7),0_2px_0_rgba(255,206,99,0.1)]",
];
const MOBILE: &[&str] = &[
    "mobile:flex",
    "mobile:flex-row",
    "mobile:sticky",
    "mobile:top-0",
    "mobile:z-[60]",
    "mobile:[background:linear-gradient(180deg,rgba(8,14,30,0.98)_0%,rgba(8,14,30,0.96)_100%)]",
    "mobile:[padding-top:max(0.5rem,env(safe-area-inset-top))]",
    "mobile:pb-2",
    "mobile:pl-2",
    "mobile:pr-2",
    "mobile:border-b-[rgba(255,206,99,0.3)]",
    "mobile:min-h-14",
    "mobile:max-w-full",
    "mobile:w-full",
];
const TABLET: &[&str] = &[
    "tablet:flex",
    "tablet:flex-row",
    "tablet:sticky",
    "tablet:top-0",
    "tablet:z-[60]",
    "tablet:[background:linear-gradient(180deg,rgba(8,14,30,0.98)_0%,rgba(8,14,30,0.96)_100%)]",
    "tablet:[padding-top:max(0.5rem,env(safe-area-inset-top))]",
    "tablet:pb-2",
    "tablet:pl-2",
    "tablet:pr-2",
    "tablet:border-b-[rgba(255,206,99,0.3)]",
    "tablet:min-h-14",
    "tablet:max-w-full",
    "tablet:w-full",
];
const LAPTOP: &[&str] = &[
    "laptop:grid",
    "laptop:[grid-template-columns:minmax(0,1fr)_auto_minmax(0,1fr)]",
    "laptop:gap-6",
    "laptop:p-0",
    "laptop:pb-[1.75rem]",
];
const DESKTOP: &[&str] = &[
    "desktop:grid",
    "desktop:[grid-template-columns:minmax(0,1fr)_auto_minmax(0,1fr)]",
    "desktop:gap-6",
    "desktop:p-0",
    "desktop:pb-[1.75rem]",
];
const QHD: &[&str] = &[
    "qhd:grid",
    "qhd:[grid-template-columns:minmax(0,1fr)_auto_minmax(0,1fr)]",
    "qhd:gap-6",
    "qhd:p-0",
    "qhd:pb-[1.75rem]",
];
const UHD: &[&str] = &[
    "uhd:grid",
    "uhd:[grid-template-columns:minmax(0,1fr)_auto_minmax(0,1fr)]",
    "uhd:gap-6",
    "uhd:p-0",
    "uhd:pb-[1.75rem]",
];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
