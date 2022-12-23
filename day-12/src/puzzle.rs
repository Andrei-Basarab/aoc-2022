//! --- Day 12: Hill Climbing Algorithm ---

pub mod puzzle {
    extern crate queues;
    use queues::*;

    pub static PUZZLE_NAME: &str = "--- Day 12: Hill Climbing Algorithm ---";

    pub static QUESTION_ONE: &str =
        "What is the fewest steps required to move from your current position to the location that should get the best signal?";

    pub static QUESTION_TWO: &str = "What is the fewest steps required to move starting from any square with elevation a to the location that should get the best signal?";

    #[derive(Debug, Copy)]
    struct Point {
        x: usize,
        y: usize,
        height: i32,
        visited: bool,
        parent_x: usize,
        parent_y: usize,
    }

    impl Default for Point {
        fn default() -> Point {
            Point {
                x: 0,
                y: 0,
                height: 0,
                visited: false,
                parent_x: 0,
                parent_y: 0,
            }
        }
    }

    impl Clone for Point {
        fn clone(&self) -> Point {
            let value = *self;
            value
        }
    }

    #[derive(Debug)]
    struct Heightmap {
        map: Vec<Vec<Point>>,
        start: Point,
        end: Point,
        max_x: usize,
        max_y: usize,
    }

    impl Default for Heightmap {
        fn default() -> Heightmap {
            Heightmap {
                map: Vec::new(),
                start: Point::default(),
                end: Point::default(),
                max_x: 0,
                max_y: 0,
            }
        }
    }

    impl Heightmap {
        fn get_adjacent_nodes(&mut self, point: Point) -> Vec<Point> {
            let mut adjacent_nodes = Vec::new();
            let dir_vector: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

            for vector in dir_vector {
                let x = point.x as isize + vector.0;
                let y = point.y as isize + vector.1;

                if (x >= 0) && (x < self.max_x as isize) && (y >= 0) && (y < self.max_y as isize) {
                    adjacent_nodes.push(self.map[y as usize][x as usize]);
                }
            }

            adjacent_nodes
        }

        fn mark_visited(&mut self, point: Point) {
            self.map[point.y][point.x].visited = true;
        }

        fn get_node(&mut self, point: Point) -> Point {
            self.map[point.y][point.x]
        }

        fn is_node_equal(&mut self, point_a: Point, point_b: Point) -> bool {
            if (point_a.x == point_b.x) && (point_a.y == point_b.y) {
                return true;
            } else {
                return false;
            }
        }

        fn register_parent(&mut self, point: Point, parent: Point) {
            self.map[point.y][point.x].parent_x = parent.x;
            self.map[point.y][point.x].parent_y = parent.y;
        }

        fn get_path(&mut self, start: Point, end: Point) -> Vec<Point> {
            let mut path: Vec<Point> = Vec::new();
            let mut node = self.get_node(end);

            while false == self.is_node_equal(start, node) {
                path.push(node);
                let mut parent = Point::default();
                parent.x = node.parent_x;
                parent.y = node.parent_y;
                node = self.get_node(parent);
            }

            path
        }

        fn find_shortest_path_start_to_end(&mut self) -> Vec<Point> {
            let mut queue: Queue<Point> = queue![self.start];
            self.mark_visited(self.start);

            while queue.size() > 0 {
                let node = queue.remove().unwrap();
                let adjacent_nodes = self.get_adjacent_nodes(node);

                for adjacent_node in adjacent_nodes {
                    if (adjacent_node.visited == false)
                        && (node.height >= (adjacent_node.height - 1))
                    {
                        self.mark_visited(adjacent_node);
                        self.register_parent(adjacent_node, node);
                        let _ = queue.add(adjacent_node);
                    }
                }
            }

            let mut path = self.get_path(self.start, self.end);
            path.reverse();

            path
        }

        fn find_shortest_path_lowest_to_end(&mut self) -> Vec<Point> {
            let mut lowest_point = Point::default();
            let mut queue: Queue<Point> = queue![self.end];
            self.mark_visited(self.end);

            'outer: while queue.size() > 0 {
                let node = queue.remove().unwrap();
                let adjacent_nodes = self.get_adjacent_nodes(node);

                for adjacent_node in adjacent_nodes {
                    if (adjacent_node.visited == false)
                        && (node.height <= (adjacent_node.height + 1))
                    {
                        self.mark_visited(adjacent_node);
                        self.register_parent(adjacent_node, node);
                        let _ = queue.add(adjacent_node);

                        if adjacent_node.height == 0 {
                            lowest_point = adjacent_node;
                            break 'outer;
                        }
                    }
                }
            }

            self.get_path(self.end, lowest_point)
        }
    }

    fn char_to_value(char: char) -> u8 {
        let start_end: char;

        if char.is_uppercase() {
            start_end = match char {
                'S' => 'a',
                'E' => 'z',
                _ => panic!("Unknown value: {}", char),
            }
        } else {
            start_end = char;
        }

        // 'a' => 0; 'z' => 25
        (start_end as u8) - 97
    }

    fn parse_input_file(file_content: &String) -> Heightmap {
        let lines = file_content.lines();
        let mut heightmap: Heightmap = Heightmap::default();
        let mut char_matrix: Vec<Vec<char>> = Vec::new();

        // Convert input content into matrix of chars
        for line in lines {
            char_matrix.push(line.chars().collect());
        }

        heightmap.max_x = char_matrix[0].len();
        heightmap.max_y = char_matrix.len();

        // Convert matrix of chars into Heightmap. i = lines, j = columns
        for i in 0..heightmap.max_y {
            heightmap.map.push(Vec::new());
            for j in 0..heightmap.max_x {
                let char = char_matrix[i][j];
                let point = Point {
                    x: j,
                    y: i,
                    height: char_to_value(char) as i32,
                    visited: false,
                    parent_x: 0,
                    parent_y: 0,
                };
                heightmap.map[i].push(point.clone());

                if char == 'S' {
                    heightmap.start = point;
                } else if char == 'E' {
                    heightmap.end = point;
                }
            }
        }

        heightmap
    }

    #[allow(dead_code)]
    fn print_heightmap(heightmap: &Heightmap) {
        for i in 0..heightmap.map.len() {
            for j in 0..heightmap.map[i].len() {
                print!("{:2}", heightmap.map[i][j].height);
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print_heightmap_path(heightmap: &mut Heightmap, path: Vec<Point>) {
        let mut char_heightmap = vec![vec![' '; heightmap.max_x]; heightmap.max_y];

        for point in path {
            if true == heightmap.is_node_equal(heightmap.start, point) {
                char_heightmap[point.y][point.x] = 'S';
            } else if true == heightmap.is_node_equal(heightmap.end, point) {
                char_heightmap[point.y][point.x] = 'E';
            } else {
                char_heightmap[point.y][point.x] = '*';
            }
        }

        for i in 0..heightmap.max_y {
            for j in 0..heightmap.max_x {
                print!("{}", char_heightmap[i][j]);
            }
            println!();
        }
    }

    pub fn solve_part_one(file_content: &String) -> String {
        let mut heightmap = parse_input_file(file_content);

        let path = heightmap.find_shortest_path_start_to_end();

        path.len().to_string()
    }

    pub fn solve_part_two(file_content: &String) -> String {
        let mut heightmap = parse_input_file(file_content);

        let path = heightmap.find_shortest_path_lowest_to_end();

        path.len().to_string()
    }
}
