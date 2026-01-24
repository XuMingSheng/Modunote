use uuid::Uuid;

#[derive(Debug)]
pub struct CreateBlockDto {
    pub id: Uuid,
    pub title: String,
    pub content: String,
}

#[derive(Debug)]
pub struct UpdateBlockDto {
    pub title: Option<String>,
    pub content: Option<String>,
}
