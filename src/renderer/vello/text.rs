use std::{num::NonZeroU32, rc::Rc};

use crate::{backend::renderer_data::vello::VelloRendererData, types::{ParagraphTrait, TextOptions}};

#[derive(Debug, Clone)]
pub struct VelloParagraph {
    options: TextOptions
}

impl ParagraphTrait for Rc<VelloParagraph> {
    fn options(&self) -> &TextOptions {
        &self.options
    }

    fn height(&self) -> u32 {
        100
    }
}

impl VelloParagraph {
    pub fn new(data: &VelloRendererData, text: String, width: u32, max_height: Option<NonZeroU32>, options: TextOptions) -> Self {
        Self {
            options
        }
    }
}