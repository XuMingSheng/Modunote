use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OpenedBlock {
    pub block_id: Uuid,
    pub opened_at: DateTime<Utc>,
    pub tab_order: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub opened_blocks: Vec<OpenedBlock>,
}

impl Workspace {
    pub fn open_block(&mut self, block_id: Uuid) {
        if self.find_opened_block_index(block_id).is_some() {
            return;
        }

        let next_order = (self.opened_blocks.len() + 1) as u32;

        let new_opened_block = OpenedBlock {
            block_id,
            opened_at: Utc::now(),
            tab_order: next_order,
        };

        self.opened_blocks.push(new_opened_block)
    }

    pub fn close_block(&mut self, block_id: Uuid) {
        let Some(index) = self.find_opened_block_index(block_id) else {
            return;
        };

        self.opened_blocks.remove(index);

        self.compact_opened_block_orders();
    }
}

impl Workspace {
    fn find_opened_block_index(&self, block_id: Uuid) -> Option<usize> {
        self.opened_blocks
            .iter()
            .position(|b| b.block_id == block_id)
    }

    fn compact_opened_block_orders(&mut self) {
        for (index, block) in self.opened_blocks.iter_mut().enumerate() {
            block.tab_order = index as u32;
        }
    }
}
