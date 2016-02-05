fn apply_presents_to_house(house_num: &usize, min_presents: &usize) -> bool {
    let mut present_count = 0;
    for n in 1..(house_num + 1) {
        if house_num % n == 0 {
            present_count += n * 10;
        }
    }
    present_count >= *min_presents
}

fn main() {
    let min_presents = 29000000;

    let mut house_num = 1;

    loop {
        if apply_presents_to_house(&house_num, &min_presents) {
            break
        } else {
            house_num += 1;
        }
    }

    println!("{:?}", house_num);
}
