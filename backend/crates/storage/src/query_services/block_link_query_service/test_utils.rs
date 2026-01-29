use sqlx::{Acquire, Database, Executor};
use uuid::Uuid;

use domain::blocks::Block;

use super::BlockLinkQueryServiceResult as Result;
use crate::query_services::BlockLinkQueryService;
use crate::repositories::{
    BlockDirectionalLinkRepository, BlockRelatedLinkRepository, BlockRepository,
    block_directional_link_repository::CreateBlockDirectionalLinkDto,
    block_related_link_repository::CreateBlockRelatedLinkDto,
};

struct SeededGraph {
    target_id: Uuid,
    parent_ids: [Uuid; 2],
    child_ids: [Uuid; 2],
    related_ids: [Uuid; 2],
}

async fn seed_link_graph<'a, A, RB, RD, RR, DB>(
    block_repo: &RB,
    directional_repo: &RD,
    related_repo: &RR,
    conn: A,
) -> SeededGraph
where
    DB: Database,
    RB: BlockRepository<DB>,
    RD: BlockDirectionalLinkRepository<DB>,
    RR: BlockRelatedLinkRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let target = Block::new("target", "content");
    let parent_one = Block::new("parent one", "content");
    let parent_two = Block::new("parent two", "content");
    let child_one = Block::new("child one", "content");
    let child_two = Block::new("child two", "content");
    let related_one = Block::new("related one", "content");
    let related_two = Block::new("related two", "content");

    let mut conn = conn.acquire().await.expect("Failed to get connection");

    block_repo
        .save(&target, &mut *conn)
        .await
        .expect("failed to seed target block");
    block_repo
        .save(&parent_one, &mut *conn)
        .await
        .expect("failed to seed parent one");
    block_repo
        .save(&parent_two, &mut *conn)
        .await
        .expect("failed to seed parent two");
    block_repo
        .save(&child_one, &mut *conn)
        .await
        .expect("failed to seed child one");
    block_repo
        .save(&child_two, &mut *conn)
        .await
        .expect("failed to seed child two");
    block_repo
        .save(&related_one, &mut *conn)
        .await
        .expect("failed to seed related one");
    block_repo
        .save(&related_two, &mut *conn)
        .await
        .expect("failed to seed related two");

    directional_repo
        .create(
            &CreateBlockDirectionalLinkDto {
                id: Uuid::new_v4(),
                block_from_id: parent_one.id,
                block_to_id: target.id,
            },
            &mut *conn,
        )
        .await
        .expect("failed to create parent one link");
    directional_repo
        .create(
            &CreateBlockDirectionalLinkDto {
                id: Uuid::new_v4(),
                block_from_id: parent_two.id,
                block_to_id: target.id,
            },
            &mut *conn,
        )
        .await
        .expect("failed to create parent two link");

    directional_repo
        .create(
            &CreateBlockDirectionalLinkDto {
                id: Uuid::new_v4(),
                block_from_id: target.id,
                block_to_id: child_one.id,
            },
            &mut *conn,
        )
        .await
        .expect("failed to create child one link");
    directional_repo
        .create(
            &CreateBlockDirectionalLinkDto {
                id: Uuid::new_v4(),
                block_from_id: target.id,
                block_to_id: child_two.id,
            },
            &mut *conn,
        )
        .await
        .expect("failed to create child two link");

    related_repo
        .create(
            &CreateBlockRelatedLinkDto {
                id: Uuid::new_v4(),
                block_a_id: target.id,
                block_b_id: related_one.id,
            },
            &mut *conn,
        )
        .await
        .expect("failed to create related one link");
    related_repo
        .create(
            &CreateBlockRelatedLinkDto {
                id: Uuid::new_v4(),
                block_a_id: target.id,
                block_b_id: related_two.id,
            },
            &mut *conn,
        )
        .await
        .expect("failed to create related two link");

    SeededGraph {
        target_id: target.id,
        parent_ids: [parent_one.id, parent_two.id],
        child_ids: [child_one.id, child_two.id],
        related_ids: [related_one.id, related_two.id],
    }
}

pub async fn assert_get_parent_blocks<'a, A, Q, BR, DR, RR, DB>(
    query_service: &Q,
    block_repo: &BR,
    directional_repo: &DR,
    related_repo: &RR,
    conn: A,
) -> Result<()>
where
    DB: Database,
    Q: BlockLinkQueryService<DB>,
    BR: BlockRepository<DB>,
    DR: BlockDirectionalLinkRepository<DB>,
    RR: BlockRelatedLinkRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let seeded = seed_link_graph(block_repo, directional_repo, related_repo, &mut tx).await;
    let parents = query_service
        .get_parent_blocks(seeded.target_id, &mut *tx)
        .await?;

    assert_eq!(parents.len(), 2);
    for parent_id in seeded.parent_ids {
        assert!(parents.iter().any(|block| block.block_id == parent_id));
    }

    tx.rollback().await?;
    Ok(())
}

pub async fn assert_get_child_blocks<'a, A, Q, BR, DR, RR, DB>(
    query_service: &Q,
    block_repo: &BR,
    directional_repo: &DR,
    related_repo: &RR,
    conn: A,
) -> Result<()>
where
    DB: Database,
    Q: BlockLinkQueryService<DB>,
    BR: BlockRepository<DB>,
    DR: BlockDirectionalLinkRepository<DB>,
    RR: BlockRelatedLinkRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let seeded = seed_link_graph(block_repo, directional_repo, related_repo, &mut tx).await;
    let children = query_service
        .get_child_blocks(seeded.target_id, &mut *tx)
        .await?;

    assert_eq!(children.len(), 2);
    for child_id in seeded.child_ids {
        assert!(children.iter().any(|block| block.block_id == child_id));
    }

    tx.rollback().await?;
    Ok(())
}

pub async fn assert_get_related_blocks<'a, A, Q, BR, DR, RR, DB>(
    query_service: &Q,
    block_repo: &BR,
    directional_repo: &DR,
    related_repo: &RR,
    conn: A,
) -> Result<()>
where
    DB: Database,
    Q: BlockLinkQueryService<DB>,
    BR: BlockRepository<DB>,
    DR: BlockDirectionalLinkRepository<DB>,
    RR: BlockRelatedLinkRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let seeded = seed_link_graph(block_repo, directional_repo, related_repo, &mut tx).await;
    let related = query_service
        .get_related_blocks(seeded.target_id, &mut *tx)
        .await?;

    assert_eq!(related.len(), 2);
    for related_id in seeded.related_ids {
        assert!(related.iter().any(|block| block.block_id == related_id));
    }

    tx.rollback().await?;
    Ok(())
}

pub async fn assert_get_linked_blocks<'a, A, Q, BR, DR, RR, DB>(
    query_service: &Q,
    block_repo: &BR,
    directional_repo: &DR,
    related_repo: &RR,
    conn: A,
) -> Result<()>
where
    DB: Database,
    Q: BlockLinkQueryService<DB>,
    BR: BlockRepository<DB>,
    DR: BlockDirectionalLinkRepository<DB>,
    RR: BlockRelatedLinkRepository<DB>,
    A: Acquire<'a, Database = DB>,
    for<'c> &'c mut DB::Connection: Executor<'c, Database = DB> + Acquire<'c, Database = DB>,
{
    let mut conn = conn.acquire().await?;
    let mut tx = conn.begin().await?;

    let seeded = seed_link_graph(block_repo, directional_repo, related_repo, &mut tx).await;
    let linked = query_service
        .get_linked_blocks(seeded.target_id, &mut *tx)
        .await?;

    assert_eq!(linked.parent_blocks.len(), 2);
    assert_eq!(linked.child_blocks.len(), 2);
    assert_eq!(linked.related_blocks.len(), 2);

    for parent_id in seeded.parent_ids {
        assert!(
            linked
                .parent_blocks
                .iter()
                .any(|block| block.block_id == parent_id)
        );
    }
    for child_id in seeded.child_ids {
        assert!(
            linked
                .child_blocks
                .iter()
                .any(|block| block.block_id == child_id)
        );
    }
    for related_id in seeded.related_ids {
        assert!(
            linked
                .related_blocks
                .iter()
                .any(|block| block.block_id == related_id)
        );
    }

    tx.rollback().await?;
    Ok(())
}
