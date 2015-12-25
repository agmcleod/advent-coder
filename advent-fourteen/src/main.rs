use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

struct Reindeer {
    speed: usize,
    speed_duration: usize,
    rest: usize,
    distance_covered: usize,
    run_accumulation: usize,
    rest_accumulation: usize,
    running: bool,
    points: usize
}

impl Reindeer {
    fn new(speed: usize, speed_duration: usize, rest: usize) -> Reindeer {
        Reindeer{
            speed: speed,
            speed_duration: speed_duration,
            rest: rest,
            distance_covered: 0,
            rest_accumulation: 0,
            run_accumulation: 0,
            running: true,
            points: 0
        }
    }

    fn done_resting(self: &Reindeer) -> bool {
        self.rest_accumulation == self.rest
    }

    fn rest(self: &mut Reindeer) {
        self.rest_accumulation += 1;
    }

    fn run(self: &mut Reindeer) {
        self.run_accumulation += 1;
        self.distance_covered += self.speed;
        if self.run_accumulation == self.speed_duration {
            self.stop_running();
        }
    }

    fn start_running(self: &mut Reindeer) {
        self.run_accumulation = 0;
        self.running = true;
    }

    fn stop_running(self: &mut Reindeer) {
        self.running = false;
        self.rest_accumulation = 0;
    }
}

fn give_points_to_lead(reindeer_map: &mut HashMap<&str, Reindeer>) {
    let mut reindeer_to_grant = None;
    let mut distance = 0;
    for (name, reindeer) in reindeer_map.iter_mut() {
        if (reindeer.distance_covered > distance) {
            distance = reindeer.distance_covered;
            reindeer_to_grant = Some(reindeer);
        }
    }

    match reindeer_to_grant {
        Some(r) => r.points += 1,
        None => {}
    }
}

fn make_number(s: &str) -> usize {
    match s.parse() {
        Ok(n) => n,
        Err(e) => panic!("{:?}", e)
    }
}

fn read_text() -> Result<String> {
    let mut text = String::new();
    let mut file = try!(File::open("reindeer.txt"));
    try!(file.read_to_string(&mut text));
    Ok(text)
}

fn main() {
    let text = match read_text() {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e)
    };

    let mut reindeer_map: HashMap<&str, Reindeer> = HashMap::new();

    for line in text.split("\n").collect::<Vec<&str>>().iter() {
        if *line == "" {
            continue
        }
        let pieces = &line.split(" ").collect::<Vec<&str>>();

        let reindeer = Reindeer::new(make_number(pieces[3]), make_number(pieces[6]), make_number(pieces[13]));

        reindeer_map.insert(&pieces[0], reindeer);
    }

    let mut seconds = 2503i16;
    loop {
        for (name, reindeer) in reindeer_map.iter_mut() {
            if reindeer.running {
                reindeer.run();
            } else {
                reindeer.rest();
                if reindeer.done_resting() {
                    reindeer.start_running();
                }
            }
        }
        seconds -= 1;

        give_points_to_lead(&mut reindeer_map);

        if seconds < 0 {
            break;
        }
    }

    for (name, reindeer) in reindeer_map.iter() {
        println!("{} {} {}", name, reindeer.distance_covered, reindeer.points);
    }
}
