use crate::get_inputs;

pub fn fully_contains(a: &std::ops::Range<i32>, b: &std::ops::Range<i32>) -> bool {
    return a.start <= b.start && a.end >= b.end
}


///
/// ```
/// use aoc_rust::aoc_2022::day4::overlaps;
///
/// assert!(overlaps(&(1..5), &(2..8)));
/// assert!(overlaps(&(2..5), &(1..4)));
/// assert!(overlaps(&(2..3), &(1..4)));
/// assert!(overlaps(&(5..7), &(7..9)));
/// assert!(!overlaps(&(1..3), &(4..5)));
/// assert!(!overlaps(&(10..12), &(1..5)));
/// ```
pub fn overlaps(a: &std::ops::Range<i32>, b: &std::ops::Range<i32>) -> bool {
    return a.start <= b.end && a.end >= b.start
}

pub fn get_range_from_string(range: String) -> std::ops::Range<i32> {
    let mut bounds = range.split("-").into_iter();
    let first = bounds.next().expect("unable to get first");
    let second = bounds.next().expect("unable to get second");
    let lower = first.parse::<i32>().expect("unable to get lower");
    let upper = second.parse::<i32>().expect("unable to get upper");
    return lower..upper
}

pub fn day1(input: &Vec<String>) -> i32{
    let mut count = 0;
    for line in input {
        let mut ranges = line.split(",");
        let first = ranges.next().expect("missing first");
        let second = ranges.next().expect("missing second");
        let a = get_range_from_string(first.to_string());
        let b = get_range_from_string(second.to_string());
        if fully_contains(&a,&b) || fully_contains(&b, &a){
            count += 1;
        }

    }
    return count
}

pub fn day2(input: &Vec<String>) -> i32{
    let mut count = 0;
    for line in input {
        let mut ranges = line.split(",");
        let first = ranges.next().expect("missing first");
        let second = ranges.next().expect("missing second");
        let a = get_range_from_string(first.to_string());
        let b = get_range_from_string(second.to_string());
        if overlaps(&a,&b) {
            count += 1;
        }

    }
    return count
}

pub fn run() {
    let input = get_inputs(2022, 4);
    let result = day1(&input);
    println!("{:?}", result);
    let result = day2(&input);
    println!("{:?}", result);
}