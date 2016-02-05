fn apply_presents_to_house(house_num: &usize, min_presents: &usize) -> bool {
    let sqrt = (*house_num as f32).sqrt() as usize;
    let result = (2..sqrt + 1).fold(house_num + 1, |acc, i| {
        if house_num % i == 0 {
            acc + i + house_num / i
        } else {
            acc
        }
    });
    10 *
    if sqrt * sqrt == *house_num {
        result - sqrt
    } else {
        result
    } >= *min_presents
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
