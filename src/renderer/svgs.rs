use std::sync::Arc;

use uuid::Uuid;



/// ### A cheap to clone SVG container.
/// Stores:
/// * The SVG's bytes in an Arc<\[u8]>
/// * A Uuid to allow cheap lookup in for example a HashMap
#[derive(Debug, Clone)]
pub struct CacheableSvg {
    bytes: Arc<[u8]>,
    uuid: Uuid
}

impl CacheableSvg {
    pub fn new(bytes: Arc<[u8]>) -> Self {
        Self { bytes, uuid: Uuid::new_v4() }
    }

    pub fn new_cloned(bytes: &[u8]) -> Self {
        Self::new(Arc::from_iter(bytes.iter().cloned()))
    }

    pub fn bytes(&self) -> Arc<[u8]> {
        self.bytes.clone()
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}