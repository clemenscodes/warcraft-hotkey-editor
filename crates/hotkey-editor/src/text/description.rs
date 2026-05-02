use crate::text::inner_spaces::InnerSpaces;
use crate::text::level_markers::LevelMarkers;
use crate::text::substitution_placeholders::SubstitutionPlaceholders;

pub(crate) struct Description;

impl Description {
    pub(crate) fn lines_from(raw_description: &str) -> Vec<String> {
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
