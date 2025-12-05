use std::ops::Deref;

pub struct CharMap {
  buf: Vec<u8>,
  width: usize,
  height: usize,
  offset: usize,
  line_gap: usize, // the extra space between two lines
}

// A 2D rectangular character map. Every field in the map has 8
// neighbours, including the fields on the edge of the map. In other
// words, the map has a one-field wide border that is not part of the
// map, but can be seen as the neighbours of edge fields.
impl CharMap {
  // Create a CharMap from an AoC map input. Each row of the input map
  // must be terminated with a newline character, including the last
  // one. All lines must have the same width.
  pub fn new<C: TryInto<u8>>(input: &String, border: C) -> CharMap {
    let len = input.len();
    let width = input.find('\n').unwrap();
    let line_gap = 1;
    let height = len / (width + line_gap);

    // buf[0] will be the top left neighbour of input[0], so
    // buf[1] will be the top centre neighbour of input[0], so
    // input[0] has to go to buf[width + line_gap + 1]
    let offset = width + line_gap + 1;

    // The input is terminated by a newline (the line gap), so we only
    // need width + 1 more bytes after it in the buf.
    let buf_size = offset + len + width + 1;

    let c = border.try_into().ok().unwrap();
    let mut buf = vec![c; buf_size];
    buf[offset..offset + len].copy_from_slice(input.as_bytes());
    for i in 0..height {
      buf[offset + width + i * (width + line_gap)] = c;
    }

    CharMap {
      buf: buf,
      width: width,
      height: height,
      offset: offset,
      line_gap: line_gap,
    }
  }

  pub fn iter(&self) -> CharMapIter<'_> {
    CharMapIter {
      map: &self,
      x: 0,
      y: 0,
    }
  }

  pub fn neighbours(&self, index: (usize, usize)) -> Vec<u8> {
    let offset = self.offset(index);
    let line_step = self.width + self.line_gap;
    [
      &self.buf[offset-line_step-1..offset-line_step+2],
      &self.buf[offset-1..offset],
      &self.buf[offset+1..offset+2],
      &self.buf[offset+line_step-1..offset+line_step+2],
    ].concat()
  }

  fn offset(&self, index: (usize, usize)) -> usize {
    assert!(index.0 < self.width);
    assert!(index.1 < self.height);
    self.offset + index.0 + index.1 * (self.width + self.line_gap)
  }
}

impl std::ops::Index<(usize, usize)> for CharMap {
  type Output = u8;

  fn index(&self, index: (usize, usize)) -> &Self::Output {
    &self.buf[self.offset(index)]
  }
}

impl std::ops::IndexMut<(usize, usize)> for CharMap {
  fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
    assert!(index.0 < self.width);
    assert!(index.1 < self.height);

    &mut self.buf[self.offset + index.0 + index.1 * (self.width + self.line_gap)]
  }
}

pub struct CharMapIter<'a> {
  map: &'a CharMap,
  x: usize,
  y: usize,
}

impl<'a> std::iter::Iterator for CharMapIter<'a> {
  type Item = Field<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.y >= self.map.height {
      None
    } else {
      let field = Field{
        map: self.map,
        x: self.x,
        y: self.y,
      };

      self.x += 1;
      if self.x == self.map.width {
        self.x = 0;
        self.y += 1;
      }

      Some(field)
    }
  }
}

pub struct Field<'a> {
  map: &'a CharMap,
  x: usize,
  y: usize,
}

impl<'a> Field<'a> {
  pub fn coords(&self) -> (usize, usize) {
    (self.x, self.y)
  }

  pub fn neighbours(&self) -> Vec<u8> {
    self.map.neighbours(self.coords())
  }
}

impl<'a> PartialEq<char> for Field<'a> {
  fn eq(&self, other: &char) -> bool {
    *self.deref() == *other as u8
  }
}

impl<'a> Deref for Field<'a> {
  type Target = u8;
  fn deref(&self) -> &'a u8 {
    &self.map[self.coords()]
  }
}
