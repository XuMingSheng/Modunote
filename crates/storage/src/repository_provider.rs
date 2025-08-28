use crate::repositories::*;

#[cfg(feature = "sqlite")]
use sqlx::SqlitePool as Pool;

#[derive(Debug, Clone)]
pub struct RepositoryProvider {
    pub blocks: BlockRepository,
    pub block_pins: BlockPinRepository,
    pub block_opens: BlockOpenRepository,
    pub block_directional_links: BlockDirectionalLinkRepository,
    pub block_related_links: BlockRelatedLinkRepository,
}

impl RepositoryProvider {
    pub fn new(pool: &Pool) -> Self {
        Self {
            blocks: BlockRepository::new(pool),
            block_opens: BlockOpenRepository::new(pool),
            block_pins: BlockPinRepository::new(pool),
            block_directional_links: BlockDirectionalLinkRepository::new(pool),
            block_related_links: BlockRelatedLinkRepository::new(pool),
        }
    }
}
