const INPUT: &'static str = "L1, L5, R1, R3, L4, L5, R5, R1, L2, L2, L3, R4, L2, R3, R1, L2, R5, R3, L4, R4, L3, R3, R3, L2, R1, L3, R2, L1, R4, L2, R4, L4, R5, L3, R1, R1, L1, L3, L2, R1, R3, R2, L1, R4, L4, R2, L189, L4, R5, R3, L1, R47, R4, R1, R3, L3, L3, L2, R70, L1, R4, R185, R5, L4, L5, R4, L1, L4, R5, L3, R2, R3, L5, L3, R5, L1, R5, L4, R1, R2, L2, L5, L2, R4, L3, R5, R1, L5, L4, L3, R4, L3, L4, L1, L5, L5, R5, L5, L2, L1, L2, L4, L1, L2, R3, R1, R1, L2, L5, R2, L3, L5, L4, L2, L1, L2, R3, L1, L4, R3, R3, L2, R5, L1, L3, L3, L3, L5, R5, R1, R2, L3, L2, R4, R1, R1, R3, R4, R3, L3, R3, L5, R2, L2, R4, R5, L4, L3, L1, L5, L1, R1, R2, L1, R3, R4, R5, R2, R3, L2, L1, L5";

enum Heading {
    North,
    East,
    South,
    West
}

struct Location {
    x: i32,
    y: i32
}

static mut HEADING: Heading = Heading::North;
static mut LOCATION: Location = Location {x: 0, y: 0};

fn main() {
    let steps: Vec<&str> = INPUT.split(", ").collect();
        
    unsafe {
        for step in &steps {
            match step.split_at(1) {
                ("L", dist) => {
                    turn_left();
                    move_dist(dist);
                },
                ("R", dist) => {
                    turn_right();
                    move_dist(dist);
                },
                _ => {}
            }
        }

        println!("({},{}); {} total blocks", LOCATION.x, LOCATION.y, LOCATION.x.abs() + LOCATION.y.abs());
    }
}

unsafe fn turn_left() {
    HEADING = match HEADING {
        Heading::North => Heading::West,
        Heading::West => Heading::South,
        Heading::South => Heading::East,
        Heading::East => Heading::North
    }
}

unsafe fn turn_right() {
    HEADING = match HEADING {
        Heading::West => Heading::North,
        Heading::North => Heading::East,
        Heading::East => Heading::South,
        Heading::South => Heading::West
    }
}

unsafe fn move_dist(dist: &str) {
    let dist_int = match dist.parse::<i32>() {
        Ok(val) => val,
        Err(_) => 0
    };

    match HEADING {
        Heading::North => LOCATION.y += dist_int,
        Heading::West  => LOCATION.x -= dist_int,
        Heading::South => LOCATION.y -= dist_int,
        Heading::East  => LOCATION.x += dist_int
    }
}
