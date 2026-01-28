use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::OpenedBlock;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Workspace {
    pub opened_blocks: Vec<OpenedBlock>,
}

impl Workspace {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_block_opened(&self, block_id: Uuid) -> bool {
        self.find_opened_block_index(block_id).is_some()
    }

    pub fn open_block(&mut self, block_id: Uuid) {
        if self.is_block_opened(block_id) {
            return;
        }

        let next_index = self.opened_blocks.len() + 1;

        let new_opened_block = OpenedBlock {
            block_id,
            opened_at: Utc::now(),
            tab_index: next_index,
        };

        self.opened_blocks.push(new_opened_block)
    }

    pub fn close_block(&mut self, block_id: Uuid) {
        let Some(index) = self.find_opened_block_index(block_id) else {
            return;
        };

        self.opened_blocks.remove(index);

        self.compact_opened_block_indices();
    }
}

impl Workspace {
    fn find_opened_block_index(&self, block_id: Uuid) -> Option<usize> {
        self.opened_blocks
            .iter()
            .position(|b| b.block_id == block_id)
    }

    fn compact_opened_block_indices(&mut self) {
        for (index, block) in self.opened_blocks.iter_mut().enumerate() {
            block.tab_index = index;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Workspace;
    use uuid::Uuid;

    #[test]
    fn open_block_adds_unique_entries() {
        let mut workspace = Workspace {
            opened_blocks: Vec::new(),
        };

        let first = Uuid::new_v4();
        let second = Uuid::new_v4();

        workspace.open_block(first);
        workspace.open_block(second);
        workspace.open_block(first);

        assert_eq!(workspace.opened_blocks.len(), 2);
        assert_eq!(workspace.opened_blocks[0].block_id, first);
        assert_eq!(workspace.opened_blocks[1].block_id, second);
    }

    #[test]
    fn close_block_removes_and_compacts_orders() {
        let mut workspace = Workspace {
            opened_blocks: Vec::new(),
        };

        let first = Uuid::new_v4();
        let second = Uuid::new_v4();

        workspace.open_block(first);
        workspace.open_block(second);

        workspace.close_block(first);

        assert_eq!(workspace.opened_blocks.len(), 1);
        assert_eq!(workspace.opened_blocks[0].block_id, second);
        assert_eq!(workspace.opened_blocks[0].tab_index, 0);
    }

    #[test]
    fn close_block_missing_is_noop() {
        let mut workspace = Workspace {
            opened_blocks: Vec::new(),
        };

        let existing = Uuid::new_v4();
        let missing = Uuid::new_v4();

        workspace.open_block(existing);
        workspace.close_block(missing);

        assert_eq!(workspace.opened_blocks.len(), 1);
        assert_eq!(workspace.opened_blocks[0].block_id, existing);
    }
}
