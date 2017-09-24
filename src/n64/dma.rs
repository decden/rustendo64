use std::cmp;

#[derive(Default, Debug)]
pub struct DMARequest {
    pub from: u32,
    pub to: u32,
    pub length: u32,
}

impl DMARequest {
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        self.length != 0
    }

    pub fn get_chunk(&mut self, chunk_size: u32) -> DMARequest {
        let chunk = DMARequest {
            from: self.from,
            to: self.to,
            length: cmp::min(self.length, chunk_size),
        };
        self.length -= chunk.length;
        self.from += chunk.length;
        self.to += chunk.length;

        chunk
    }
}