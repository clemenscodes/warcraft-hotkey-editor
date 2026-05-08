pub(super) struct ModeButtonClass;

impl ModeButtonClass {
    pub(super) fn get() -> &'static str {
        "flex-1 min-h-[2.75rem] px-6 bg-[linear-gradient(180deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)] border border-[#6c5a1f] rounded-[10px] text-[#c0c8da] font-friz-quadrata text-[1.8rem] uppercase tracking-[0.08em] [text-shadow:1px_1px_0_#000] transition-[border-color,color,box-shadow] duration-150 hover:border-warcraft-gold hover:text-warcraft-gold focus:outline-none [body[data-kb-modality]_&]:focus:outline-none [body[data-kb-modality]_&]:focus:border-white [body[data-kb-modality]_&]:focus:text-white [body[data-kb-modality]_&]:focus:shadow-[0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)] data-[active=true]:bg-[linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)] data-[active=true]:border-warcraft-gold data-[active=true]:text-warcraft-gold data-[active=true]:shadow-[0_0_12px_rgba(255,206,99,0.3)] min-[701px]:max-[2000px]:text-[clamp(1rem,0.5vw+0.7rem,1.4rem)] min-[701px]:max-[2000px]:px-4 max-[700px]:min-h-[3.5rem] max-[700px]:text-[1.15rem] max-[700px]:px-[0.85rem] max-[480px]:text-[1rem] max-[480px]:min-h-[3.2rem] max-[480px]:px-[0.6rem]"
    }
}
