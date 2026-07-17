use tw_macro::tw;

// The four independent toggles — both modes and both catalog toggles — share one
// row, because they are one kind of control. Their labels differ in length, so
// they keep their natural width and a long one moves to the next line rather
// than squeezing the whole row down to its smallest member.
classes! {
    base: tw![
        "flex",
        "flex-wrap",
        "gap-1.5",
    ],
}
