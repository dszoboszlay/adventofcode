use crate::Day;

pub fn solvers() -> Day {
    (Some(p01), Some(p02))
}

struct Biterator<'a> {
    stream: std::str::Bytes<'a>,
    curr_val: usize,
    curr_bits: usize,
}

impl<'a> Biterator<'a> {
    fn new(input: &'a String) -> Biterator<'a> {
        Biterator {
            stream: input.bytes(),
            curr_val: 0,
            curr_bits: 0,
        }
    }

    fn fetch(&mut self, bits: usize) -> usize {
        const CA: usize = 'A' as usize;
        const C0: usize = '0' as usize;

        let mut ret = 0;
        let mut bits = bits;
        while bits > 0 {
            if self.curr_bits == 0 {
                self.curr_bits = 4;
                self.curr_val = match self.stream.next().unwrap() as usize {
                    hex if hex >= CA => hex - CA + 10,
                    dec => dec - C0
                }
            }

            if bits >= self.curr_bits {
                ret = (ret << self.curr_bits) + self.curr_val;
                bits -= self.curr_bits;
                self.curr_bits = 0;
            } else {
                let bits_to_keep = self.curr_bits - bits;
                ret = (ret << bits) + (self.curr_val >> bits_to_keep);
                bits = 0;
                self.curr_val &= (1 << bits_to_keep) - 1;
                self.curr_bits = bits_to_keep;
            }
        }

        ret
    }

    fn skip(&mut self, bits: usize) {
        if self.curr_bits > bits {
            self.curr_bits -= bits;
            self.curr_val &= (1 << self.curr_bits) - 1;   
        } else {
            let mut bits = bits - self.curr_bits;
            while bits > 4 {
                self.stream.next();
                bits -= 4;
            }

            self.curr_bits = 0;
            self.fetch(bits);
        }
    }
}

fn sum_versions(input: &mut Biterator, sum: &mut usize) -> usize {
    let mut consumed = 6; // header is always 6 bits
    *sum += input.fetch(3);
    if input.fetch(3) == 4 {
        // Literal value packet
        while input.fetch(1) == 1 {
            consumed += 5;
            input.skip(4);
        }
        consumed += 5;
        input.skip(4);
    } else if input.fetch(1) == 0 {
        // Sub-packets with given total length
        let mut sub_packet_size = input.fetch(15);
        consumed += 16 + sub_packet_size;
        while sub_packet_size > 0 {
            sub_packet_size -= sum_versions(input, sum);
        }
    } else {
        // Given number of sub-packets
        let mut sub_packet_count = input.fetch(11);
        consumed += 12;
        while sub_packet_count > 0 {
            consumed += sum_versions(input, sum);
            sub_packet_count -= 1;
        }
    }

    consumed
}

fn eval(input: &mut Biterator) -> (usize, usize) {
    let mut consumed = 6; // header is always 6 bits
    let mut val = 0;
    input.skip(3);

    let type_id = input.fetch(3);
    if type_id == 4 {
        // Literal value packet
        while input.fetch(1) == 1 {
            consumed += 5;
            val = (val << 4) + input.fetch(4);
        }
        consumed += 5;
        val = (val << 4) + input.fetch(4);
    } else {
        let mut sub_vals = Vec::new();
        if input.fetch(1) == 0 {
            // Sub-packets with given total length
            let mut sub_packet_size = input.fetch(15);
            consumed += 16 + sub_packet_size;
            while sub_packet_size > 0 {
                let (sub_val, sub_consumed) = eval(input);
                sub_vals.push(sub_val);
                sub_packet_size -= sub_consumed;
            }
         } else {
            // Given number of sub-packets
            let mut sub_packet_count = input.fetch(11);
            consumed += 12;
            while sub_packet_count > 0 {
                let (sub_val, sub_consumed) = eval(input);
                sub_vals.push(sub_val);
                consumed += sub_consumed;
                sub_packet_count -= 1;
            }
        }

        val = match type_id {
            0 => sub_vals.iter().sum(),
            1 => sub_vals.iter().product(),
            2 => *sub_vals.iter().min().unwrap(),
            3 => *sub_vals.iter().max().unwrap(),
            5 => (sub_vals[0] > sub_vals[1]) as usize,
            6 => (sub_vals[0] < sub_vals[1]) as usize,
            7 => (sub_vals[0] == sub_vals[1]) as usize,
            _ => panic!("invalid op"),
        }
    }

    (val, consumed)
}

pub fn p01(input: &String) -> String {
    let mut input = Biterator::new(input);
    let mut sum = 0;

    sum_versions(&mut input, &mut sum);
    sum.to_string()
}

pub fn p02(input: &String) -> String {
    let mut input = Biterator::new(input);
    let (val, _) = eval(&mut input);

    val.to_string()
}
