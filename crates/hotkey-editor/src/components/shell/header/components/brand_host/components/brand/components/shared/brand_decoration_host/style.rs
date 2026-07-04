use crate::classes;

// The flourish's box, sized as a container-query length off the brand host, so it
// scales in step with the title as one drawing. `[container-type:inline-size]` also
// makes this box the query context the img's own `cqi` width resolves against.
const BASE: &[&str] = &[
    "block",
    "flex-none",
    "[container-type:inline-size]",
    "w-[11cqi]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
