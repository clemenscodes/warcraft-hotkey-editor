use tw_macro::ClassList;

/// The shared-key badge wrapper; `data-top` nudges it down when it caps a stack.
pub(super) const BADGE: ClassList =
    ClassList::new("self-start h-18 inline-flex items-center justify-center data-[top=true]:mb-2");
