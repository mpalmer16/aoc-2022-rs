use aoc_common::fetch_with_transform;

fn main() {
    let section_assignments: Vec<SectionAssignment> = fetch_with_transform(4, section_assignments);

    let complete_overlaps = section_assignments
        .iter()
        .filter(|&ap| complete_overlap(ap))
        .count();

    println!("answer 1: {}", complete_overlaps);

    let any_overlaps = section_assignments
        .iter()
        .filter(|&ap| any_overlap(ap))
        .count();

    println!("answer 2: {}", any_overlaps);
}

type SectionAssignment = ((usize, usize), (usize, usize));

fn section_assignments(input: String) -> Vec<SectionAssignment> {
    input
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|range| {
                    let parts: Vec<usize> = range
                        .split('-')
                        .map(|range_bound| range_bound.parse().unwrap())
                        .collect();
                    (parts[0], parts[1])
                })
                .collect()
        })
        .map(|ranges: Vec<(usize, usize)>| (ranges[0], ranges[1]))
        .collect()
}

fn complete_overlap(((r1_start, r1_end), (r2_start, r2_end)): &SectionAssignment) -> bool {
    r1_start <= r2_start && r1_end >= r2_end || r2_start <= r1_start && r2_end >= r1_end
}

fn any_overlap(((r1_start, r1_end), (r2_start, r2_end)): &SectionAssignment) -> bool {
    for n in *r1_start..=*r1_end {
        if (*r2_start..=*r2_end).contains(&n) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {

    use aoc_common::get_test_input;

    use crate::{any_overlap, complete_overlap, section_assignments, SectionAssignment};

    const TEST_FILE: &str = "inputs/test_input.txt";

    #[test]
    fn can_get_input() {
        let section_assignments: Vec<SectionAssignment> =
            get_test_input(TEST_FILE, section_assignments);

        assert!(section_assignments.len() == 6);
    }

    #[test]
    fn can_find_complete_overlaps() {
        let section_assignments: Vec<SectionAssignment> =
            get_test_input(TEST_FILE, section_assignments);

        assert!(
            section_assignments
                .iter()
                .filter(|&ap| complete_overlap(ap))
                .count()
                == 2
        );
    }

    #[test]
    fn can_find_any_overlaps() {
        let section_assignments: Vec<SectionAssignment> =
            get_test_input(TEST_FILE, section_assignments);

        assert!(
            section_assignments
                .iter()
                .filter(|&ap| any_overlap(ap))
                .count()
                == 4
        );
    }
}
