use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

fn mark_visible(trees: &[u8], visible: &mut Vec<bool>, start: usize, stop: usize, step: usize) {
    let mut h = '0' as u8 - 1;
    let mut i = start;

    while i != stop && h < '9' as u8 {
        if trees[i] > h {
            visible[i] = true;
            h = trees[i];
        }
        i = i.wrapping_add(step);
    }
}

pub fn p01(input: &String) -> String {
    let trees = input.as_bytes();
    let mut visible = vec![false; trees.len()];
    let width = input.lines().next().unwrap().len();
    let line_len = width + 1;

    let mut x1 = 0;
    let mut x2 = trees.len() - line_len;
    while x1 < line_len {
        mark_visible(trees, &mut visible, x1, x2, line_len);
        mark_visible(trees, &mut visible, x2, x1, 0usize.wrapping_sub(line_len));
        x1 += 1;
        x2 += 1;
    }
    
    let mut y1 = 0;
    let mut y2 = width - 1;
    while y1 < trees.len() {
        mark_visible(trees, &mut visible, y1, y2, 1);
        mark_visible(trees, &mut visible, y2, y1, 0usize.wrapping_sub(1));
        y1 += line_len;
        y2 += line_len;
    }

    return visible.iter().filter(|&&v| v).count().to_string();
}
pub fn p02(input: &String) -> String {
    let trees = input.as_bytes();
    let width = input.lines().next().unwrap().len();
    let line_len = width + 1;

    let mut best = 0;
    let mut row_start = line_len;
    let mut row_end = 2 * width; // row_start + width - 1 = width + 1 + width - 1 = 2 * width
    while row_start < trees.len() {
        for i in (row_start + 1)..row_end {
            let h = trees[i];
            let mut view_up = 0;
            let mut view_down = 0;
            let mut view_left = 0;
            let mut view_right = 0;

            let mut j = i -line_len;
            while j < trees.len() {
                view_up += 1;
                if trees[j] >= h {
                    break;
                }
                j = j.wrapping_sub(line_len);
            }

            let mut j = i + line_len;
            while j < trees.len() {
                view_down += 1;
                if trees[j] >= h {
                    break;
                }
                j += line_len;
            }

            let mut j = i - 1;
            while j >= row_start {
                view_left += 1;
                if trees[j] >= h {
                    break;
                }
                j -= 1;
            }

            let mut j = i + 1;
            while j <= row_end {
                view_right += 1;
                if trees[j] >= h {
                    break;
                }
                j += 1;
            }

            best = std::cmp::max(best, view_up * view_down * view_left * view_right);
        }
        row_start += line_len;
        row_end += line_len;
    }

    return best.to_string();
}
