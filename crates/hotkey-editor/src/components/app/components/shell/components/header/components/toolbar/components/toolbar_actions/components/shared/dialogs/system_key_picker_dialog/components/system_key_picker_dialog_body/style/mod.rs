use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex-1",
        "min-h-0",
        "flex",
        "flex-col",
        "gap-6",
        "pt-10",
        "px-12",
        "pb-10",
        "overflow-y-auto",
    ],
    mobile: tw![
        "mobile:justify-center",
        "mobile:pt-5",
        "mobile:px-4",
        "mobile:pb-6",
        // A full keyboard, not the twelve-key board. Its labels run to "Backspace"
        // and "Num7", so the slots are sized off those rather than off the row
        // count, and the rows wrap onto as many lines as that costs.
        "mobile:[--key-slot:13.7cqi]",
        "mobile:[--key-slot-wide:21.2cqi]",
        "mobile:[--key-height:10.6cqi]",
        "mobile:[--key-font:3.6cqi]",
        "mobile:[--key-row-justify:flex-start]",
    ],
}
