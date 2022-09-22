use arrayvec::ArrayVec;

#[derive(Debug)]
pub struct Display {
    patterns: ArrayVec<String, { Self::NUM_PATTERNS }>,
    outputs: ArrayVec<String, { Self::NUM_OUTPUTS }>,
}

impl Display {
    const NUM_PATTERNS: usize = 10;
    const NUM_OUTPUTS: usize = 4;
    const UNIQUE_SEGMENTS: [i32; 4] = [2, 3, 4, 7];

    pub fn from_str(input: &str) -> Self {
        let input = input.split(" | ").collect::<Vec<_>>();
        let patterns = input[0]
            .split(' ')
            .map(|e| e.to_owned())
            .collect::<ArrayVec<_, { Self::NUM_PATTERNS }>>();
        let outputs = input[1]
            .split(' ')
            .map(|e| e.to_owned())
            .collect::<ArrayVec<_, { Self::NUM_OUTPUTS }>>();
        Self { patterns, outputs }
    }

    pub fn count_unique_segments(&self) -> i32 {
        let mut count = 0;
        for output in &self.outputs {
            if Self::UNIQUE_SEGMENTS.contains(&(output.len() as i32)) {
                count += 1;
            }
        }
        count
    }
}
