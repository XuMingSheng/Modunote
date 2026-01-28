cfg_if::cfg_if! {
    if #[cfg(feature = "native")] {
        pub type DatabaseImpl = storage_sqlite::SqliteDb;

        pub type BlockRepositoryImpl = storage_sqlite::repositories::SqliteBlockRepository;
        pub type BlockDirectionalLinkRepositoryImpl = storage_sqlite::repositories::SqliteBlockDirectionalLinkRepository;
        pub type BlockRelatedLinkRepositoryImpl = storage_sqlite::repositories::SqliteBlockRelatedLinkRepository;
        pub type WorkspaceRepositoryImpl = storage_sqlite::repositories::SqliteWorkspaceRepository;

        pub type BlockQueryServiceImpl = storage_sqlite::query_services::SqliteBlockQueryService;
        pub type BlockLinkQueryServiceImpl = storage_sqlite::query_services::SqliteBlockLinkQueryService;
    } else if #[cfg(feature = "cloud")] {
        pub type DatabaseImpl = storage_postgres::PostgresDb;

        pub type BlockRepositoryImpl = storage_postgres::repositories::PostgresBlockRepository;
        pub type BlockDirectionalLinkRepositoryImpl =
            storage_postgres::repositories::PostgresBlockDirectionalLinkRepository;
        pub type BlockRelatedLinkRepositoryImpl =
            storage_postgres::repositories::PostgresBlockRelatedLinkRepository;
        pub type WorkspaceRepositoryImpl =
            storage_postgres::repositories::PostgresWorkspaceRepository;

        pub type BlockQueryServiceImpl =
            storage_postgres::query_services::PostgresBlockQueryService;
        pub type BlockLinkQueryServiceImpl =
            storage_postgres::query_services::PostgresBlockLinkQueryService;
    }
}

#[derive(Clone, Debug)]
pub struct Repositories {
    pub blocks: BlockRepositoryImpl,
    pub block_directional_links: BlockDirectionalLinkRepositoryImpl,
    pub block_related_links: BlockRelatedLinkRepositoryImpl,
    pub workspaces: WorkspaceRepositoryImpl,
}

impl Repositories {
    pub fn new() -> Self {
        Self {
            blocks: BlockRepositoryImpl::new(),
            block_directional_links: BlockDirectionalLinkRepositoryImpl::new(),
            block_related_links: BlockRelatedLinkRepositoryImpl::new(),
            workspaces: WorkspaceRepositoryImpl::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct QueryServices {
    pub blocks: BlockQueryServiceImpl,
    pub block_links: BlockLinkQueryServiceImpl,
}

impl QueryServices {
    pub fn new() -> Self {
        Self {
            blocks: BlockQueryServiceImpl::new(),
            block_links: BlockLinkQueryServiceImpl::new(),
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
        let repos = Repositories::new();
        let query_services = QueryServices::new();

        Self {
            db,
            repos,
            query_services,
        }
    }
}
