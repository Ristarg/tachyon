pub struct SourceStream {
    source: Vec<char>,
    idx: usize,
}

// this is literally a peekable iterator
// I am literally retarded
// but I have no clue how to implement this with a real Peekable
// so we're stuck with this for now
impl SourceStream {
    pub fn new(source: &str) -> SourceStream {
        SourceStream {
            //TODO: probably eventually better to keep an iterator from the original slice for perf reasons
            source: source.chars().collect(),
            idx: 0,
        }
    }

    pub fn cur_char(&self) -> Option<char> {
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
