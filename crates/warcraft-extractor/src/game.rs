use std::path::{Path, PathBuf};

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule};

pub static GAME_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: GameExtraction::matches,
    target: ExtractTarget::Raw,
    output_path: GameExtraction::output_path,
    processor: GameExtraction::process,
};

struct GameExtraction;

impl GameExtraction {
    fn matches(_: &str) -> bool {
        true
    }

    fn output_path(path: &str, base: &Path) -> PathBuf {
        base.join(path)
    }

    fn process(_: &str, _: &[u8]) -> Result<ExtractResult, ExtractError> {
        Ok(ExtractResult::IO)
    }
}
