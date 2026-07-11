use tw_macro::tw;
// The flourish's box, sized as a container-query length off the brand host, so it
// scales in step with the title as one drawing. `[container-type:inline-size]` also
// makes this box the query context the img's own `cqi` width resolves against.

classes! {
    base: tw![
        "@container",
        "block",
        "flex-none",
        "w-[11cqi]",
    ],
}
