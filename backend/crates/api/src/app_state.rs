use crate::config::{
    BlockDirectionalLinkRepositoryImpl, BlockLinkQueryServiceImpl, BlockQueryServiceImpl,
    BlockRelatedLinRepositoryImpl, BlockRepositoryImpl, DatabaseImpl, WorkspaceRepositoryImpl,
};
use storage::Database;

#[derive(Clone, Debug)]
pub struct Repositories {
    pub blocks: BlockRepositoryImpl,
    pub block_directional_links: BlockDirectionalLinkRepositoryImpl,
    pub block_related_links: BlockRelatedLinRepositoryImpl,
    pub workspaces: WorkspaceRepositoryImpl,
}

impl Repositories {
    pub fn new(db: &DatabaseImpl) -> Self {
        Self {
            blocks: BlockRepositoryImpl::new(db.pool()),
            block_directional_links: BlockDirectionalLinkRepositoryImpl::new(db.pool()),
            block_related_links: BlockRelatedLinRepositoryImpl::new(db.pool()),
            workspaces: WorkspaceRepositoryImpl::new(db.pool()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct QueryServices {
    pub blocks: BlockQueryServiceImpl,
    pub block_links: BlockLinkQueryServiceImpl,
}

impl QueryServices {
    pub fn new(db: &DatabaseImpl) -> Self {
        Self {
            blocks: BlockQueryServiceImpl::new(db.pool()),
            block_links: BlockLinkQueryServiceImpl::new(db.pool()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DatabaseImpl,
    pub repos: Repositories,
    pub query_services: QueryServices,
}

impl AppState {
    pub fn new(db: DatabaseImpl) -> Self {
        let repos = Repositories::new(&db);
        let query_services = QueryServices::new(&db);

        Self {
            db,
            repos,
            query_services,
        }
    }
}
