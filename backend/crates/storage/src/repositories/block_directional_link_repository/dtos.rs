use uuid::Uuid;

#[derive(Debug)]
pub struct CreateBlockDirectionalLinkDto {
    pub id: Uuid,
    pub block_from_id: Uuid,
    pub block_to_id: Uuid,
}
