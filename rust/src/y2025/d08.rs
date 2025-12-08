use crate::Day;

pub fn solvers() -> Day {
  (Some(p01),
   Some(p02),
  )
}

pub fn p01(input: &String) -> String {
  let coords: Vec<(i64, i64, i64)> = input.lines().map(|l| {
    let mut ns = l.split(',').map(|n| n.parse().unwrap());
    let x = ns.next().unwrap();
    let y = ns.next().unwrap();
    let z = ns.next().unwrap();
    (x, y, z)
  }).collect();

  let mut distance_squares = Vec::with_capacity(coords.len() * (coords.len() - 1) / 2);
  for i in 0..coords.len() - 1 {
    for j in i+1..coords.len() {
      let sq = (coords[i].0 - coords[j].0) * (coords[i].0 - coords[j].0) +
      (coords[i].1 - coords[j].1) * (coords[i].1 - coords[j].1) +
      (coords[i].2 - coords[j].2) * (coords[i].2 - coords[j].2);
      distance_squares.push((sq, i, j));
    }
  }
  distance_squares.sort();

  let mut circuit_sizes = vec![1; 3];
  let mut circuit_idx = vec![None; coords.len()];

  distance_squares.iter().take(1000).for_each(|&(_, i, j)| {
    match (circuit_idx[i], circuit_idx[j]) {
      (None, None) => {
        let v = Some(circuit_sizes.len());
        circuit_idx[i] = v;
        circuit_idx[j] = v;
        circuit_sizes.push(2);
      }
      (None, Some(k)) => {
        circuit_idx[i] = Some(k);
        circuit_sizes[k] += 1;
      }
      (Some(k), None) => {
        circuit_idx[j] = Some(k);
        circuit_sizes[k] += 1;
      }
      (Some(k), Some(l)) => {
        if k != l {
          circuit_sizes[k] += circuit_sizes[l];
          circuit_sizes[l] = 0;
          circuit_idx.iter_mut().filter(|&&mut x| x == Some(l)).for_each(|x| {
            *x = Some(k)
          });
        }
      }
    }
  });

  circuit_sizes.sort();
  circuit_sizes.iter().rev().take(3).fold(1, |acc, size| acc * size).to_string()
}

pub fn p02(input: &String) -> String {
  let coords: Vec<(i64, i64, i64)> = input.lines().map(|l| {
    let mut ns = l.split(',').map(|n| n.parse().unwrap());
    let x = ns.next().unwrap();
    let y = ns.next().unwrap();
    let z = ns.next().unwrap();
    (x, y, z)
  }).collect();

  let mut distance_squares = Vec::with_capacity(coords.len() * (coords.len() - 1) / 2);
  for i in 0..coords.len() - 1 {
    for j in i+1..coords.len() {
      let sq = (coords[i].0 - coords[j].0) * (coords[i].0 - coords[j].0) +
      (coords[i].1 - coords[j].1) * (coords[i].1 - coords[j].1) +
      (coords[i].2 - coords[j].2) * (coords[i].2 - coords[j].2);
      distance_squares.push((sq, i, j));
    }
  }
  distance_squares.sort();

  let mut circuit_sizes = vec![1; 3];
  let mut circuit_idx = vec![None; coords.len()];

  distance_squares.iter().fold(None, |acc, &(_, i, j)| {
    if acc.is_some() {
      return acc
    }

    let idx = match (circuit_idx[i], circuit_idx[j]) {
      (None, None) => {
        let idx = circuit_sizes.len();
        let v = Some(idx);
        circuit_idx[i] = v;
        circuit_idx[j] = v;
        circuit_sizes.push(2);
        idx
      }
      (None, Some(k)) => {
        circuit_idx[i] = Some(k);
        circuit_sizes[k] += 1;
        k
      }
      (Some(k), None) => {
        circuit_idx[j] = Some(k);
        circuit_sizes[k] += 1;
        k
      }
      (Some(k), Some(l)) => {
        if k != l {
          circuit_sizes[k] += circuit_sizes[l];
          circuit_sizes[l] = 0;
          circuit_idx.iter_mut().filter(|&&mut x| x == Some(l)).for_each(|x| {
            *x = Some(k)
          });
        }
        k
      }
    };

    if circuit_sizes[idx] == coords.len() {
      Some(coords[i].0 * coords[j].0)
    } else {
      None
    }
  }).unwrap().to_string()
}
