use tw_macro::tw;

classes! {
    base: tw![
        "fixed",
        "inset-x-[10vw]",
        "inset-y-[10vh]",
        "z-1000",
        "flex",
        "flex-col",
        "overflow-hidden",
        "rounded-container",
        "border",
        "border-warcraft-gold",
        "bg-warcraft-bg-base",
        "bg-panel-toast",
        "shadow-overlay",
        "focus:outline-none",
        // It scales and fades in on opening and out on closing; the primitive
        // marks the state on the dialog's parent.
        "motion-safe:[[data-state=open]>&]:animate-dialog-in",
        "motion-safe:[[data-state=closed]>&]:animate-dialog-out",
    ],
    mobile: tw![
        "mobile:inset-0",
        "mobile:rounded-none",
        "mobile:border-x-0",
    ],
    // From a tablet up the dialog is a centred panel as tall as its content.
    // Centred inside its 1rem insets, its automatic height never exceeds the room
    // between them; beyond that it scrolls. A dialog built around a pager, a
    // result list or the export text has no content height of its own to hug, so
    // it stretches between the insets instead.
    tablet: tw![
        "tablet:inset-4",
        "tablet:h-auto",
        "tablet:self-center",
        "tablet:has-[.template-pager,.search-dialog-body,.preview-textarea,.system-hotkeys-dialog-body]:self-stretch",
    ],
    // A laptop is wider than a dialog's content wants, so the panel is no
    // wider than 80rem, centred between its insets.
    laptop: tw![
        "laptop:inset-4",
        "laptop:w-full",
        "laptop:max-w-7xl",
        "laptop:h-auto",
        "laptop:justify-self-center",
        "laptop:self-center",
        "laptop:has-[.template-pager,.search-dialog-body,.preview-textarea,.system-hotkeys-dialog-body]:self-stretch",
    ],
    // From a desktop up every dialog is the same panel, centred: most of the
    // window wide, less of it on a 4K screen, and four fifths of it tall, so
    // switching dialogs never changes the panel. The unit detail dialog shows as
    // many stats as its unit has, so it hugs them, up to that same height.
    desktop: tw![
        "desktop:inset-0",
        "desktop:w-[85vw]",
        "desktop:h-[80vh]",
        "desktop:has-[.unit-detail-dialog-body]:h-auto",
        "desktop:has-[.unit-detail-dialog-body]:max-h-[80vh]",
        "desktop:justify-self-center",
        "desktop:self-center",
    ],
    qhd: tw![
        "qhd:inset-0",
        "qhd:w-[85vw]",
        "qhd:h-[80vh]",
        "qhd:has-[.unit-detail-dialog-body]:h-auto",
        "qhd:has-[.unit-detail-dialog-body]:max-h-[80vh]",
        "qhd:justify-self-center",
        "qhd:self-center",
    ],
    uhd: tw![
        "uhd:inset-0",
        "uhd:w-[70vw]",
        "uhd:h-[80vh]",
        "uhd:has-[.unit-detail-dialog-body]:h-auto",
        "uhd:has-[.unit-detail-dialog-body]:max-h-[80vh]",
        "uhd:justify-self-center",
        "uhd:self-center",
    ],
}
