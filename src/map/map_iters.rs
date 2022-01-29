pub struct HeightMapIter<'a> {
    array: &'a Vec<f32>,
    index: usize,
}

pub struct HeightMapNormIter<'a> {
    array: &'a Vec<f32>,
    index: usize,
    max: f32,
    min: f32,
}

impl<'a> HeightMapIter<'a> {
    pub fn new(array: &'a Vec<f32>) -> Self {
        HeightMapIter { array, index: 0 }
    }
}

impl<'a> HeightMapNormIter<'a> {
    pub fn new(array: &'a Vec<f32>, max: f32, min: f32) -> Self {
        println!("norm! max height: {}, min height: {}", max, min);
        HeightMapNormIter {
            array,
            index: 0,
            max,
            min,
        }
    }
}

impl Iterator for HeightMapIter<'_> {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let r = if self.index < self.array.len() {
            Some(self.array[self.index])
        } else {
            None
        };
        self.index += 1;
        r
    }
}

impl Iterator for HeightMapNormIter<'_> {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let r = if self.index < self.array.len() {
            let v = (self.array[self.index] - self.min) / (self.max - self.min);
            Some(v)
        } else {
            None
        };
        self.index += 1;
        r
    }
}
