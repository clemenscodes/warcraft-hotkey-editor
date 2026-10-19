use tw_macro::tw;
// A row across the whole table, right below the regeneration it explains and
// in the table's own type, so nothing in the panel changes size for it.
classes! {
    base: tw![
        "col-span-3",
        "flex",
        "items-center",
        "justify-end",
        "gap-[0.35em]",
        "text-warcraft-text-secondary",
        "[&>span]:inline-flex",
        "[&_svg]:size-[1em]",
    ],
}
