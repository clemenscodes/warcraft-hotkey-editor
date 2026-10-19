use tw_macro::tw;
// A box of its own around the button and its dialog, so the dialog's mount
// point never becomes an item of the card header and shifts the title over.
classes! {
    base: tw![
        "flex",
        "shrink-0",
    ],
}
