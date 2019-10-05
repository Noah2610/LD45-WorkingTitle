#[derive(Clone, PartialEq)]
pub enum SolidTag {
    Default,
}

impl Default for SolidTag {
    fn default() -> Self {
        SolidTag::Default
    }
}
