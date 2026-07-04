use crate::classes;

// The button's box and container-query context: it owns the size per band and marks
// itself the query container, so the button fills it (`size-full`) and every `cqi`
// length inside the button — border, radius, icon, badge — scales against this box.
const BASE: &[&str] = &[
    "inline-flex",
    "shrink-0",
    "[container-type:inline-size]",
    "w-[5rem]",
    "h-[5rem]",
];
const MOBILE: &[&str] = &["mobile:w-9", "mobile:h-9"];
const TABLET: &[&str] = &["tablet:w-9", "tablet:h-9"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
