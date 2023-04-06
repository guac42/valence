use crate::block_pos::BlockPos;
use crate::{Decode, Encode};

#[derive(Copy, Clone, Debug, Encode, Decode)]
pub struct SignEditorOpenS2c {
    pub location: BlockPos,
}
