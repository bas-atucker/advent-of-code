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

fn main() {
    let steps: Vec<&str> = INPUT.split(", ").collect();
    
    // Location history
    let mut history: Vec<(i32, i32)> = vec![];
    let mut location: Location = Location {x: 0, y: 0};
    let mut heading: Heading = Heading::North;
        
    for step in &steps {
        match step.split_at(1) {
            ("L", dist) => {
                heading = turn_left(&heading);
                if move_dist(&mut location, &heading, dist, &mut history) { break; }
            },
            ("R", dist) => {
                heading = turn_right(&heading);
                if move_dist(&mut location, &heading, dist, &mut history) { break; }
            },
            _ => {}
        }
    }

    println!("Reached a location twice!\n({},{}); {} total blocks from the starting location.", 
             location.x, location.y, location.x.abs() + location.y.abs());
}

fn check_history(history: &mut Vec<(i32, i32)>, loc: &Location) -> bool {
    let curr = (loc.x, loc.y);
    if history.contains(&curr) {
        true
    } else {
        history.push((loc.x, loc.y));
        false
    }
}

fn turn_left(heading: &Heading) -> Heading {
    match heading {
        &Heading::North => Heading::West,
        &Heading::West => Heading::South,
        &Heading::South => Heading::East,
        &Heading::East => Heading::North
    }
}

fn turn_right(heading: &Heading) -> Heading {
    match heading {
        &Heading::West => Heading::North,
        &Heading::North => Heading::East,
        &Heading::East => Heading::South,
        &Heading::South => Heading::West
    }
}

fn move_dist(mut loc: &mut Location,
             heading: &Heading,
             dist: &str, 
             mut history: &mut Vec<(i32, i32)>) -> bool {
    let dist_int = match dist.parse::<i32>() {
        Ok(val) => val,
        Err(_) => 0
    };

    for _ in 0..dist_int {
        match heading {
            &Heading::North => loc.y += 1,
            &Heading::West  => loc.x -= 1,
            &Heading::South => loc.y -= 1,
            &Heading::East  => loc.x += 1
        };

        if check_history(&mut history, &loc) { return true; }
    }

    false
}
