use crate::world::blocks::Blocks;
use crate::world::palette::Palette;

pub const SECTIONS_NUM: u16 = 16;
pub const SECTION_WIDTH: u16 = 16;
pub const SECTION_LENGTH: u16 = 16;
pub const SECTION_HEIGHT: u16 = 16;

#[derive(Debug)]
pub struct Chunk {
    sections: [Option<ChunkSection>; SECTIONS_NUM as usize],
    bitmask: i32,
    pos: ChunkPos,
    dirty: bool,
}

impl Chunk {
    pub fn empty(pos: ChunkPos) -> Self {
        Chunk {
            sections: Default::default(),
            bitmask: 0,
            pos,
            dirty: true,
        }
    }

    pub fn set_block_at(&mut self, x: u16, y: u16, z: u16, block_state: Blocks) {
        let section_y = y / SECTION_HEIGHT;
        if let Some(section) = &mut self.sections[section_y as usize] {
            section.set_block_at(x, y - (section_y * SECTION_HEIGHT), z, block_state);
            if section.block_count == 0 {
                self.sections[section_y as usize] = None;
                self.bitmask ^= 1 << section_y;
            }
        } else if block_state != Blocks::Air {
            let mut section = ChunkSection::empty();
            section.set_block_at(x, y - (section_y * SECTION_HEIGHT), z, block_state);
            self.sections[section_y as usize] = Some(section);
            self.bitmask ^= 1 << section_y;
        }
    }

    pub fn get_bit_mask(&self) -> i32 {
        self.bitmask
    }

    pub fn get_position(&self) -> &ChunkPos {
        &self.pos
    }

    pub fn mark_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn get_chunk_sections(&self) -> &[Option<ChunkSection>; SECTIONS_NUM as usize] {
        &self.sections
    }
}

#[derive(Clone, Debug)]
pub struct ChunkSection {
    block_count: u16,
    palette: Palette<Blocks>,
    blocks: Vec<u16>,
}

impl ChunkSection {
    pub fn empty() -> Self {
        ChunkSection {
            block_count: 0,
            palette: Palette::new(vec![Blocks::Air]),
            blocks: vec![0u16; (SECTION_WIDTH * SECTION_HEIGHT * SECTION_LENGTH) as usize],
        }
    }

    pub fn set_block_at(&mut self, x: u16, y: u16, z: u16, block_state: Blocks) {
        let old_value = self.blocks[Self::calculate_index(x, y, z)];
        if block_state == Blocks::Air && old_value != 0 {
            self.block_count -= 1;
            if !self.blocks.contains(&old_value) {
                let last = self.palette.remove(old_value as usize) as u16;
                for value in self.blocks.iter_mut() {
                    if *value == last {
                        *value = old_value;
                    }
                }
            }
        } else if block_state != Blocks::Air && old_value == 0 {
            self.block_count += 1;
        }
        if let Some(index) = self.palette.get_index(&block_state) {
            self.blocks[(x + z * SECTION_WIDTH + y * SECTION_WIDTH * SECTION_LENGTH) as usize] = index as u16;
        } else {
            let index = self.palette.push(block_state);
            self.blocks[(x + z * SECTION_WIDTH + y * SECTION_WIDTH * SECTION_LENGTH) as usize] = index as u16;
        }
    }

    pub fn block_at(&self, x: u16, y: u16, z: u16) -> &Blocks {
        self.palette.at(self.blocks[ChunkSection::calculate_index(x, y, z)] as usize).unwrap()
    }

    /// This is the block-count internal to `FalconMC`. Depending on the version
    /// this count will not be correct due to missing blocks. Do not depend on this number when working
    /// on networking synchronization!!
    pub fn get_block_count(&self) -> u16 {
        self.block_count
    }

    pub fn get_palette(&self) -> &Palette<Blocks> {
        &self.palette
    }

    pub fn get_block_data(&self) -> &Vec<u16> {
        &self.blocks
    }

    pub fn calculate_index(x: u16, y: u16, z: u16) -> usize {
        (x + z * SECTION_WIDTH + y * SECTION_WIDTH * SECTION_LENGTH) as usize
    }
}

impl Default for ChunkSection {
    fn default() -> Self {
        ChunkSection::empty()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    pub fn new(x: i32, z: i32) -> Self {
        ChunkPos { x, z }
    }
}

impl From<ChunkPos> for (i32, i32) {
    fn from(pos: ChunkPos) -> Self {
        (pos.x, pos.z)
    }
}

impl From<&ChunkPos> for (i32, i32) {
    fn from(pos: &ChunkPos) -> Self {
        (pos.x, pos.z)
    }
}

impl From<(i32, i32)> for ChunkPos {
    fn from((x, z): (i32, i32)) -> Self {
        ChunkPos::new(x, z)
    }
}
