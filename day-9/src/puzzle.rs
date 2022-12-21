//! --- Day 9: Rope Bridge ---

pub mod puzzle {
    pub static PUZZLE_NAME: &str = "--- Day 9: Rope Bridge ---";

    pub static QUESTION_ONE: &str = "Simulate your complete hypothetical series of motions. \
    How many positions does the tail of the rope visit at least once?";

    pub static QUESTION_TWO: &str =
        "Simulate your complete series of motions on a larger rope with ten knots. \
    How many positions does the tail of the rope visit at least once?";

    const ROPE_LEN: usize = 9;

    #[derive(Debug, Clone, Copy)]
    enum Direction {
        L,
        D,
        R,
        U,
    }

    impl Direction {
        fn from_char(value: char) -> Direction {
            match value {
                'L' => Direction::L,
                'D' => Direction::D,
                'R' => Direction::R,
                'U' => Direction::U,
                _ => panic!("Unknown value: {}", value),
            }
        }

        // Get tuple (x, y)
        fn get_vector(value: &Direction) -> (isize, isize) {
            match value {
                Direction::L => (-1, 0),
                Direction::D => (0, 1),
                Direction::R => (1, 0),
                Direction::U => (0, -1),
            }
        }
    }

    #[derive(Debug, Copy)]
    struct Motion {
        direction: Direction,
        steps: usize,
    }

    impl Clone for Motion {
        fn clone(&self) -> Motion {
            let value = *self;
            value
        }
    }

    #[derive(Debug, Copy)]
    struct Point {
        start: bool,
        head: bool,
        rope: [bool; ROPE_LEN],
        tail_visited: bool,
    }

    impl Default for Point {
        fn default() -> Point {
            Point {
                start: false,
                head: false,
                rope: [false; ROPE_LEN],
                tail_visited: false,
            }
        }
    }

    impl Clone for Point {
        fn clone(&self) -> Point {
            let value = *self;
            value
        }
    }

    struct Area {
        grid: Vec<Vec<Point>>,
        head_loc: (usize, usize),
        rope_loc: [(usize, usize); ROPE_LEN],
    }

    impl Default for Area {
        fn default() -> Area {
            Area {
                grid: Vec::new(),
                head_loc: (0, 0),             // (x, y)
                rope_loc: [(0, 0); ROPE_LEN], // (x, y)
            }
        }
    }

    enum Rope {
        Head,
        None,
    }

    fn parse_input_file(file_content: &String) -> Vec<Motion> {
        let lines = file_content.lines();
        let mut motion_series: Vec<Motion> = Vec::new();

        for line in lines {
            let motion_set: Vec<&str> = line.split_whitespace().collect();
            motion_series.push(Motion {
                direction: Direction::from_char(motion_set[0].chars().next().unwrap()),
                steps: motion_set[1].parse::<usize>().unwrap(),
            });
        }

        motion_series
    }

    fn find(area: &Area, rope_knot: Rope, knot_index: usize) -> (usize, usize) {
        let max_y = area.grid.len();
        let max_x = area.grid[0].len();
        let mut y = 0;
        let mut x = 0;

        for i in 0..max_y {
            for j in 0..max_x {
                let knot;
                let point = area.grid[i][j];

                if knot_index < ROPE_LEN {
                    knot = point.rope[knot_index];
                } else {
                    match rope_knot {
                        Rope::Head => {
                            knot = point.head;
                        }
                        Rope::None => {
                            knot = false;
                        }
                    }
                }

                if knot == true {
                    x = j;
                    y = i;
                    break;
                }
            }
        }

        (x, y)
    }

    fn motion_is_possible(area: &Area, motion: &Motion) -> (bool, Motion) {
        let max_y = area.grid.len();
        let max_x = area.grid[0].len();
        let steps = motion.steps;
        let mut motion_possible = true;
        let mut delta_motion = (*motion).clone();
        let (x, y) = area.head_loc;

        match motion.direction {
            Direction::L => {
                if x < steps {
                    motion_possible = false;
                    delta_motion.steps -= x;
                }
            }
            Direction::D => {
                if (y + steps) >= max_y {
                    motion_possible = false;
                    delta_motion.steps = steps - (max_y - y - 1);
                }
            }
            Direction::R => {
                if (x + steps) >= max_x {
                    motion_possible = false;
                    delta_motion.steps = steps - (max_x - x - 1);
                }
            }
            Direction::U => {
                if y < steps {
                    motion_possible = false;
                    delta_motion.steps -= y;
                }
            }
        }

        (motion_possible, delta_motion)
    }

    fn extend_area<'a>(area: &'a mut Area, motion: &Motion) {
        let max_y = area.grid.len();
        let max_x = area.grid[0].len();

        match motion.direction {
            Direction::L => {
                for i in 0..max_y {
                    for _ in 0..motion.steps {
                        area.grid[i].insert(0, Point::default());
                    }
                }
            }
            Direction::D => {
                for _ in 0..motion.steps {
                    area.grid.push(vec![Point::default(); max_x]);
                }
            }
            Direction::R => {
                for i in 0..max_y {
                    for _ in 0..motion.steps {
                        area.grid[i].push(Point::default());
                    }
                }
            }
            Direction::U => {
                for _ in 0..motion.steps {
                    area.grid.insert(0, vec![Point::default(); max_x]);
                }
            }
        }

        // Update Head and Tail locations
        area.head_loc = find(area, Rope::Head, ROPE_LEN);

        // Update whole Rope's Knots locations
        for i in 0..ROPE_LEN {
            area.rope_loc[i] = find(area, Rope::None, i);
        }
    }

    fn move_knot(head_knot: (usize, usize), tail_knot: (usize, usize)) -> (usize, usize) {
        let (head_x, head_y) = head_knot;
        let (mut tail_x, mut tail_y) = tail_knot;
        let (mut delta_x, mut delta_y) = (
            head_x as isize - tail_x as isize,
            head_y as isize - tail_y as isize,
        );

        if (delta_x.abs() > 1) || (delta_y.abs() > 1) {
            if delta_x != 0 {
                delta_x /= delta_x.abs();
                tail_x = (tail_x as isize + delta_x) as usize;
            }

            if delta_y != 0 {
                delta_y /= delta_y.abs();
                tail_y = (tail_y as isize + delta_y) as usize;
            }
        }

        (tail_x, tail_y)
    }

    fn perform_motion<'a>(area: &'a mut Area, motion: &Motion, knots: u32) {
        let mut head_knot_loc = area.head_loc;
        let tail_knots = knots as usize - 1;

        for _ in 0..motion.steps {
            let vector = Direction::get_vector(&motion.direction);

            // Move [H]ead
            area.grid[head_knot_loc.1][head_knot_loc.0].head = false;
            head_knot_loc = (
                (head_knot_loc.0 as isize + vector.0) as usize,
                (head_knot_loc.1 as isize + vector.1) as usize,
            );
            area.grid[head_knot_loc.1][head_knot_loc.0].head = true;
            area.head_loc = head_knot_loc;

            let mut prev_knot_loc = head_knot_loc;

            for i in 0..tail_knots {
                let mut current_knot_loc = area.rope_loc[i];

                // Attempt to move [T]ail/Knot
                area.grid[current_knot_loc.1][current_knot_loc.0].rope[i] = false;
                current_knot_loc = move_knot(prev_knot_loc, current_knot_loc);
                area.grid[current_knot_loc.1][current_knot_loc.0].rope[i] = true;

                // Update knot location
                area.rope_loc[i] = current_knot_loc;

                prev_knot_loc = current_knot_loc;

                // Check if this knot is Tail
                if i == (tail_knots - 1) {
                    // Set tail_visited to true at new tail location
                    area.grid[current_knot_loc.1][current_knot_loc.0].tail_visited = true;
                }
            }
        }
    }

    fn perform_motions(mut area: Area, motion_series: &Vec<Motion>, knots: u32) -> Area {
        // Initialize the first Point
        area.grid.push(vec![Point {
            start: true,
            head: true,
            rope: [true; ROPE_LEN],
            tail_visited: true,
        }]);

        for motion in motion_series {
            let (motion_possible, delta_motion) = motion_is_possible(&mut area, motion);
            if motion_possible == false {
                extend_area(&mut area, &delta_motion);
            }
            perform_motion(&mut area, motion, knots);
        }

        area
    }

    fn count_visited_cells(area: &Area) -> u32 {
        let mut cells = 0;

        for line in &area.grid {
            for point in line {
                if point.tail_visited == true {
                    cells += 1;
                }
            }
        }

        cells
    }

    #[allow(dead_code)]
    fn print_area(area: &Area) {
        for line in &area.grid {
            for point in line {
                if point.head == true {
                    print!("H");
                // } else if point.tail == true {
                //     print!("T");
                } else if point.start == true {
                    print!("s");
                } else if point.tail_visited == true {
                    print!("#");
                } else {
                    let mut symbol = ".".to_string();
                    for i in 0..ROPE_LEN {
                        if point.rope[i] == true {
                            symbol = i.to_string();
                        }
                    }
                    print!("{}", symbol);
                }
            }
            println!();
        }
    }

    pub fn solve_part_one(file_content: &String) -> u32 {
        let motion_series = parse_input_file(&file_content);
        let mut area = Area::default();

        area = perform_motions(area, &motion_series, 2);

        count_visited_cells(&area)
    }

    pub fn solve_part_two(file_content: &String) -> u32 {
        let motion_series = parse_input_file(&file_content);
        let mut area = Area::default();

        area = perform_motions(area, &motion_series, 10);

        count_visited_cells(&area)
    }
}
