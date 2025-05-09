use std::rc::Rc;

use crate::types::{ParagraphTrait, TextOptions};

#[derive(Debug, Clone)]
pub struct VelloParagraph {}

impl ParagraphTrait for Rc<VelloParagraph> {
    fn options(&self) -> &TextOptions {
        todo!()
    }

    fn height(&self) -> u32 {
        todo!()
    }
}
