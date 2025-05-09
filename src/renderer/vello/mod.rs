use crate::types::{Window, RendererData, Object, RResult};

use super::RendererTrait;


pub mod text;


#[derive(Debug)]
pub struct VelloRenderer {}

impl VelloRenderer {
    pub fn new() -> RResult<Self> {
        Ok(VelloRenderer {
        
        })
    }
}

impl RendererTrait for VelloRenderer {
    fn render(&self, window: &Window, data: &RendererData, objects: Vec<&Object>) -> RResult<()> {
        todo!()
    }

    fn recreate(&self, window: &Window) {
        todo!()
    }
}
