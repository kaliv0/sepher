use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

//TODO: should we put helper structs below main one?
#[derive(Clone, Debug)]
pub struct TextFragment {
    pub grapheme: String,
    pub rendered_width: GraphemeWidth,
    pub replacement: Option<char>,
    pub start: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum GraphemeWidth {
    Half,
    Full,
}

//////////////////////////////
#[derive(Default, Clone)]
pub struct Line {
    fragments: Vec<TextFragment>,
    string: String,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        // debug_assert!(line_str.is_empty() || line_str.lines().count() == 1);
        let fragments = Self::str_to_fragments(line_str);
        Self {
            fragments,
            string: String::from(line_str),
        }
    }

    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        line_str
            .grapheme_indices(true)
            .map(|(byte_idx, grapheme)| {
                let (replacement, rendered_width) = Self::get_replacement_character(grapheme)
                    .map_or_else(
                        || {
                            let unicode_width = grapheme.width();
                            let rendered_width = match unicode_width {
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };
                            (None, rendered_width)
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Half),
                    );

                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                    start: byte_idx,
                }
            })
            .collect()
    }

    fn get_replacement_character(for_str: &str) -> Option<char> {
        let width = for_str.width();
        match for_str {
            " " => None,
            "\t" => Some(' '),
            //TODO: refactor as nested match?
            _ if width > 0 && for_str.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next() {
                    if ch.is_control() && chars.next().is_none() {
                        return Some('▯');
                    }
                }
                Some('·')
            }
            _ => None,
        }
    }

    ///////////////////////
    pub fn width(&self) -> usize {
        self.width_until(self.grapheme_count())
    }

    pub fn width_until(&self, grapheme_idx: usize) -> usize {
        self.fragments
            .iter()
            .take(grapheme_idx)
            .map(|fragment| match fragment.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            })
            .sum()
    }

    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    // pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
    //     self.get_annotated_visible_substr(range, None).to_string()
    // }
    //
    // pub fn get_annotated_visible_substr(
    //     &self,
    //     range: Range<usize>,
    //     annotations: Option<&Vec<Annotation>>,
    // ) -> AnnotatedString {
    //     if range.start >= range.end {
    //         return AnnotatedString::default();
    //     }
    //     // Create a new annotated string
    //     let mut result = AnnotatedString::from(&self.string);
    //
    //     // Apply annotations for this string
    //     if let Some(annotations) = annotations {
    //         for annotation in annotations {
    //             result.add_annotation(annotation.annotation_type, annotation.start, annotation.end);
    //         }
    //     }
    //
    //     // Insert replacement characters, and truncate if needed.
    //     // We do this backwards, otherwise the byte indices would be off
    //     // in case a replacement character has a different width than the original character.
    //     let mut fragment_start = self.width();
    //     for fragment in self.fragments.iter().rev() {
    //         let fragment_end = fragment_start;
    //         fragment_start = fragment_start.saturating_sub(fragment.rendered_width.into());
    //
    //         if fragment_start > range.end {
    //             continue; // No  processing needed if we haven't reached the visible range yet.
    //         }
    //
    //         // clip right if the fragment is partially visible
    //         if fragment_start < range.end && fragment_end > range.end {
    //             result.replace(fragment.start, self.string.len(), "⋯");
    //             continue;
    //         } else if fragment_start == range.end {
    //             // Truncate right if we've reached the end of the visible range
    //             result.truncate_right_from(fragment.start);
    //             continue;
    //         }
    //
    //         // Fragment ends at the start of the range: Remove the entire left side of the string (if not already at start of string)
    //         if fragment_end <= range.start {
    //             result.truncate_left_until(fragment.start.saturating_add(fragment.grapheme.len()));
    //             break; //End processing since all remaining fragments will be invisible.
    //         } else if fragment_start < range.start && fragment_end > range.start {
    //             // Fragment overlaps with the start of range: Remove the left side of the string and add an ellipsis
    //             result.replace(
    //                 0,
    //                 fragment.start.saturating_add(fragment.grapheme.len()),
    //                 "⋯",
    //             );
    //             break; //End processing since all remaining fragments will be invisible.
    //         }
    //
    //         // Fragment is fully within range: Apply replacement characters if appropriate
    //         if fragment_start >= range.start && fragment_end <= range.end {
    //             if let Some(replacement) = fragment.replacement {
    //                 let start = fragment.start;
    //                 let end = start.saturating_add(fragment.grapheme.len());
    //                 result.replace(start, end, &replacement.to_string());
    //             }
    //         }
    //     }
    //     result
    // }
}
