pub struct SourceStream {
    source: Vec<u8>,
    idx: usize,
}

// this is literally a peekable iterator
// I am literally retarded
// but I have no clue how to implement this with a real Peekable
// so we're stuck with this for now
impl SourceStream {
    pub fn new(source: &str) -> SourceStream {
        SourceStream {
            source: source.to_owned().into_bytes(),
            idx: 0,
        }
    }

    pub fn cur_char(&self) -> Option<u8> {
        if self.idx == self.source.len() {
            return None;
        }

        Some(self.source[self.idx])
    }

    pub fn advance(&mut self) {
        if self.idx < self.source.len() {
            self.idx += 1;
        }
    }
}
