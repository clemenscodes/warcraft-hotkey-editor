pub(super) struct UnitCategoryHeadingClass;

impl UnitCategoryHeadingClass {
    pub(super) fn compute(is_collapsed: bool) -> &'static str {
        if is_collapsed {
            "font-friz-quadrata text-[1.2rem] text-[#5a6075] uppercase tracking-[0.12em] mt-3 mb-1 py-[0.35rem] px-[0.25rem] border-b border-[#1f3d63] w-full text-left flex items-center gap-[0.4rem] cursor-pointer transition-colors duration-[120ms] first:mt-0 hover:text-warcraft-gold focus:outline-none [body[data-kb-modality]_&]:focus:outline-none [body[data-kb-modality]_&]:focus:text-warcraft-gold"
        } else {
            "font-friz-quadrata text-[1.2rem] text-[#7b818d] uppercase tracking-[0.12em] mt-3 mb-1 py-[0.35rem] px-[0.25rem] border-b border-[#1f3d63] w-full text-left flex items-center gap-[0.4rem] cursor-pointer transition-colors duration-[120ms] first:mt-0 hover:text-warcraft-gold focus:outline-none [body[data-kb-modality]_&]:focus:outline-none [body[data-kb-modality]_&]:focus:text-warcraft-gold"
        }
    }
}
