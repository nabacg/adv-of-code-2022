use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

struct MarkerDetector {
    ring_buffer: VecDeque<char>,
    chars_processed: usize,
    marker_length: usize,
}

impl MarkerDetector {
    fn new(l: usize) -> MarkerDetector {
        return MarkerDetector {
            marker_length: l,
            ring_buffer: VecDeque::with_capacity(l),
            chars_processed: 0,
        };
    }

    fn process(&mut self, c: char) -> bool {
        if self.ring_buffer.len() >= self.marker_length {
            self.ring_buffer.pop_front();
        }
        self.ring_buffer.push_back(c);

        self.chars_processed += 1;
        HashSet::<_>::from_iter(self.ring_buffer.iter()).len() == self.marker_length
    }
}

pub fn result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let marker_len = 14;
    for l in lines {
        let mut sop = MarkerDetector::new(marker_len);
        l.chars().take_while(|&c| !sop.process(c)).count();
        println!("Result: {}", sop.chars_processed);
    }
    Ok(())
}
