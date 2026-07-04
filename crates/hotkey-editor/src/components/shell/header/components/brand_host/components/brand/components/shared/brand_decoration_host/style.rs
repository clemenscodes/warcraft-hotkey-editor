use crate::{classes, styling::TailwindClass, tw};

// The flourish's box, sized as a container-query length off the brand host, so it
// scales in step with the title as one drawing. `[container-type:inline-size]` also
// makes this box the query context the img's own `cqi` width resolves against.
const BASE: &[TailwindClass] = tw![
    "block",
    "flex-none",
    "[container-type:inline-size]",
    "w-[11cqi]",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
