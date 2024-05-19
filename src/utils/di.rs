use crate::utils::db::DbPool;

#[derive(Clone)]
pub struct DiContainer {}

impl DiContainer {
    pub fn new(pool: &DbPool) -> Self {
        Self {}
    }
}
