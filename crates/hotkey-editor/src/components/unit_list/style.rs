use crate::classes;

// The unit list panel. On the sidebar it is an absolutely-positioned column filling
// the sidebar column of the editor workspace; on small screens it collapses to a
// static, self-contained block (search + tabs + horizontal card carousel). `group`
// lets the cards read the active category and the scrollbar reveal on hover. The
// per-band sidebar widths match the workspace grid's first column.
const BASE: &[&str] = &[
    "group",
    "flex",
    "flex-col",
    "gap-2",
    "overflow-hidden",
    "min-w-0",
    "min-h-0",
];

const MOBILE: &[&str] = &[
    "mobile:static",
    "mobile:w-full",
    "mobile:max-h-none",
    "mobile:overflow-visible",
    "mobile:gap-4",
    "mobile:p-0",
    "mobile:bg-transparent",
    "mobile:border-0",
    "mobile:[contain:layout]",
];

const TABLET: &[&str] = &[
    "tablet:static",
    "tablet:w-full",
    "tablet:max-h-none",
    "tablet:overflow-visible",
    "tablet:gap-4",
    "tablet:p-0",
    "tablet:bg-transparent",
    "tablet:border-0",
    "tablet:[contain:layout]",
];

const LAPTOP: &[&str] = &[
    "laptop:absolute",
    "laptop:top-0",
    "laptop:left-0",
    "laptop:w-[34rem]",
    "laptop:h-full",
];

const DESKTOP: &[&str] = &[
    "desktop:absolute",
    "desktop:top-0",
    "desktop:left-0",
    "desktop:w-[34rem]",
    "desktop:h-full",
];

const QHD: &[&str] = &[
    "qhd:absolute",
    "qhd:top-0",
    "qhd:left-0",
    "qhd:w-[46rem]",
    "qhd:h-full",
];

const UHD: &[&str] = &[
    "uhd:absolute",
    "uhd:top-0",
    "uhd:left-0",
    "uhd:w-[62rem]",
    "uhd:h-full",
];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
