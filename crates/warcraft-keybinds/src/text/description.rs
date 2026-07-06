use super::inner_spaces::InnerSpaces;
use super::level_markers::LevelMarkers;
use super::substitution_placeholders::SubstitutionPlaceholders;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Description;

impl Description {
    pub fn lines_from(raw_description: &str) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        let with_newlines = raw_description.replace("|n", "\n");
        let placeholders_stripped = SubstitutionPlaceholders::stripped(&with_newlines);
        for raw_line in placeholders_stripped.lines() {
            let level_markers_stripped = LevelMarkers::stripped(raw_line);
            let trimmed_line = level_markers_stripped.trim();
            let collapsed_line = InnerSpaces::collapsed(trimmed_line);
            if collapsed_line.is_empty() {
                continue;
            }
            if let Some(last_line) = output.last()
                && last_line == &collapsed_line
            {
                continue;
            }
            output.push(collapsed_line);
        }
        output
    }
}
