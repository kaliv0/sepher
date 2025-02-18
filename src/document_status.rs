#[derive(Default, Eq, PartialEq, Debug)]
pub struct DocumentStatus {
    pub total_lines: usize,
    pub current_line_idx: usize,
    pub is_modified: bool,
    pub file_name: String,
}
