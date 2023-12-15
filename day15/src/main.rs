fn hash(input: &str) -> u8 {
    input
        .as_bytes()
        .iter()
        .fold(0 as u8, |cur, ch| cur.wrapping_add(*ch).wrapping_mul(17))
}

#[test]
fn test_hash() {
    assert_eq!(hash("HASH"), 52);
    assert_eq!(hash("rn=1"), 30);
    assert_eq!(hash("rn"), 0);
}

fn part1(input: &str) -> u64 {
    input.split(',').map(|s| hash(s) as u64).sum()
}

#[test]
fn test_part1() {
    assert_eq!(
        part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        1320
    );
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

struct Boxes(Vec<Vec<Lens>>);

impl Boxes {
    fn delete(&mut self, label: &str) {
        let b = &mut self.0[hash(label) as usize];
        if let Some(pos) = b.iter().position(|lens| lens.label == label) {
            b.remove(pos);
        }
    }

    fn insert(&mut self, label: &str, focal_length: u8) {
        let b = &mut self.0[hash(label) as usize];
        if let Some(old_lens) = b.iter_mut().find(|lens| lens.label == label) {
            old_lens.focal_length = focal_length;
        } else {
            b.push(Lens {
                label: label.to_string(),
                focal_length,
            });
        }
    }

    fn apply(&mut self, instruction: &str) {
        if instruction.ends_with('-') {
            self.delete(&instruction[0..instruction.len() - 1]);
        } else {
            let mut split = instruction.split('=');
            let label = split.next().expect("Thing before =");
            let focal_length: u8 = split
                .next()
                .expect("Thing after =")
                .parse()
                .expect("Focal length should be a number");
            self.insert(label, focal_length);
        }
    }

    fn apply_list(&mut self, list: &str) {
        list.split(',').for_each(|instruction| self.apply(instruction));
    }

    fn get_focusing_power(&self) -> u64 {
        self.0
            .iter()
            .enumerate()
            .map(|(box_idx, b)| {
                b.iter()
                    .enumerate()
                    .map(|(lens_idx, lens)| {
                        (1 + box_idx as u64) * (1 + lens_idx as u64) * (lens.focal_length as u64)
                    })
                    .sum::<u64>()
            })
            .sum()
    }
}

fn part2(input: &str) -> u64 {
    let mut boxes = Boxes(vec![vec![]; 256]);
    boxes.apply_list(input);
    boxes.get_focusing_power()
}

#[test]
fn test_part2() {
    assert_eq!(
        part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
        145
    );
}

fn main() {
    let input = include_str!("input.txt");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}
