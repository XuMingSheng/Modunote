use uuid::Uuid;

#[derive(Debug)]
pub struct CreateBlockRelatedLinkDto {
    pub id: Uuid,
    pub block_a_id: Uuid,
    pub block_b_id: Uuid,
}
