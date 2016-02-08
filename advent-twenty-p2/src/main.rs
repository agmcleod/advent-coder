fn apply_presents_to_house(house_num: &usize, min_presents: &usize, elf_start: &mut usize) -> bool {
    let result = (*elf_start..*house_num + 1).fold(0, |acc, i| {
        if house_num % i == 0 {
            if house_num / i <= 50 {
                acc + i
            } else {
                *elf_start = i;
                acc
            }

        } else {
            acc
        }
    });
    11 * result >= *min_presents
}

fn main() {
    let min_presents = 29000000;

    let mut house_num = 1;
    let mut elf_start = 1;

    loop {
        if apply_presents_to_house(&house_num, &min_presents, &mut elf_start) {
            break
        } else {
            house_num += 1;
        }
    }

    println!("{:?}", house_num);
}
