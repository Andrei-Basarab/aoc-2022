//! --- Day 10: Cathode-Ray Tube ---

pub mod puzzle {
    pub static PUZZLE_NAME: &str = "--- Day 10: Cathode-Ray Tube ---";

    pub static QUESTION_ONE: &str =
        "Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles. \
    What is the sum of these six signal strengths?";

    pub static QUESTION_TWO: &str =
        "Render the image given by your program. What eight capital letters appear on your CRT?";

    #[derive(Debug, Copy)]
    enum Instruction {
        NOOP,
        ADDX,
    }

    impl Instruction {
        fn from_str(value: &str) -> Instruction {
            match value {
                "noop" => Instruction::NOOP,
                "addx" => Instruction::ADDX,
                _ => panic!("Unknown value: {}", value),
            }
        }

        fn get_cycles(value: Instruction) -> u32 {
            match value {
                Instruction::NOOP => 1,
                Instruction::ADDX => 2,
            }
        }
    }

    impl Clone for Instruction {
        fn clone(&self) -> Instruction {
            let value = *self;
            value
        }
    }

    #[derive(Debug, Copy)]
    struct InstructionSet {
        instruction: Instruction,
        value: i32,
    }

    impl Clone for InstructionSet {
        fn clone(&self) -> InstructionSet {
            let value = *self;
            value
        }
    }

    #[derive(Debug)]
    struct Probe {
        cycles: Vec<u32>,
        cycle_index: usize,
        signal_strength: Vec<(u32, i32)>,
    }

    #[derive(Debug)]
    struct Cpu {
        program: Vec<InstructionSet>,
        cycles: u32,
        x: i32,
        pipe: InstructionSet,
        instruction_cycles: u32,
        next_instruction_index: usize,
        busy: bool,
        probe: Probe,
        crt: Crt,
    }

    impl Default for Cpu {
        fn default() -> Cpu {
            Cpu {
                program: Vec::new(),
                cycles: 0,
                x: 1,
                pipe: InstructionSet {
                    instruction: Instruction::NOOP,
                    value: 0,
                },
                instruction_cycles: 0,
                next_instruction_index: 0,
                busy: false,
                probe: Probe {
                    cycles: Vec::new(),
                    cycle_index: 0,
                    signal_strength: Vec::new(),
                },
                crt: Crt {
                    grid: Vec::new(),
                    max_x: 0,
                    max_y: 0,
                },
            }
        }
    }

    impl Cpu {
        fn load_program(&mut self, program: Vec<InstructionSet>) {
            self.program = program;
        }

        fn load_probe(&mut self, probe_cycles: Vec<u32>) {
            self.probe.cycles = probe_cycles;
        }

        fn create_crt_screen(&mut self, x: usize, y: usize) {
            self.crt = Crt {
                grid: vec![vec![' '; x]; y],
                max_x: x,
                max_y: y,
            };
        }

        fn run(&mut self, probe_enabled: bool, crt_screen_enabled: bool) {
            while (self.next_instruction_index < self.program.len())
                || ((self.next_instruction_index == self.program.len()) && (self.busy == true))
            {
                self.fetch_instruction();
                self.process_instruction();
                self.run_cycle();
                if probe_enabled == true {
                    self.probe_signal_strength();
                }
                if crt_screen_enabled == true {
                    self.draw_crt_screen();
                }
                self.execute_instruction();
            }
        }

        fn fetch_instruction(&mut self) {
            let instruction_set = self.program[self.next_instruction_index];

            if self.busy == false {
                self.pipe = instruction_set;
                self.instruction_cycles = Instruction::get_cycles(instruction_set.instruction);
                self.next_instruction_index += 1;
            }
        }

        fn process_instruction(&mut self) {
            if self.instruction_cycles != 0 {
                self.instruction_cycles -= 1;
            }
        }

        fn run_cycle(&mut self) {
            self.cycles += 1;
        }

        // Store signal strengths
        fn probe_signal_strength(&mut self) {
            if (self.probe.cycle_index < self.probe.cycles.len())
                && (self.cycles == self.probe.cycles[self.probe.cycle_index])
            {
                self.probe.cycle_index += 1;
                self.probe.signal_strength.push((self.cycles, self.x));
            }
        }

        fn draw_crt_screen(&mut self) {
            let i = (self.cycles as usize - 1) / self.crt.max_x;
            let j = (self.cycles as usize - 1) % self.crt.max_x;
            let cursor = j as i32;

            if (cursor == (self.x - 1)) || (cursor == (self.x + 0)) || (cursor == (self.x + 1)) {
                self.crt.grid[i][j] = '#';
            } else {
                self.crt.grid[i][j] = '.';
            }
        }

        fn execute_instruction(&mut self) {
            self.busy = true;

            if self.instruction_cycles == 0 {
                match self.pipe.instruction {
                    Instruction::NOOP => {}
                    Instruction::ADDX => {
                        self.x += self.pipe.value;
                    }
                }
                self.busy = false;
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Crt {
        grid: Vec<Vec<char>>,
        max_x: usize,
        max_y: usize,
    }

    fn get_sum_of_signal_strength(signal_strength: Vec<(u32, i32)>) -> u32 {
        let mut sum = 0;

        for signal in signal_strength {
            sum += signal.0 as i32 * signal.1;
        }

        sum as u32
    }

    fn parse_input_file(file_content: &String) -> Vec<InstructionSet> {
        let lines = file_content.lines();
        let mut program: Vec<InstructionSet> = Vec::new();

        for line in lines {
            let instruction_set: Vec<&str> = line.split_whitespace().collect();
            let instruction = Instruction::from_str(instruction_set[0]);
            let value;

            if instruction_set.len() == 2 {
                value = instruction_set[1].parse::<i32>().unwrap();
            } else {
                value = 0;
            }

            program.push(InstructionSet { instruction, value });
        }

        program
    }

    #[allow(dead_code)]
    fn print_program(program: &Vec<InstructionSet>) {
        for instruction_set in program {
            println!("{:?}", instruction_set);
        }
    }

    #[allow(dead_code)]
    fn print_crt_screen(crt_screen: Crt) {
        for line in crt_screen.grid {
            for pixel in line {
                print!("{}", pixel);
            }
            println!();
        }
    }

    fn crt_screen_to_string(crt_screen: Crt) -> String {
        let mut output = String::new();

        for line in crt_screen.grid {
            output = format!("{}{}{}", output, "\n", String::from_iter(line));
        }

        output
    }

    pub fn solve_part_one(file_content: &String) -> String {
        let program = parse_input_file(&file_content);
        let probe_cycles = vec![20, 60, 100, 140, 180, 220];
        let mut cpu = Cpu::default();

        cpu.load_program(program);
        cpu.load_probe(probe_cycles);
        cpu.run(true, false);

        get_sum_of_signal_strength(cpu.probe.signal_strength).to_string()
    }

    pub fn solve_part_two(file_content: &String) -> String {
        let program = parse_input_file(&file_content);
        let mut cpu = Cpu::default();

        cpu.load_program(program);
        cpu.create_crt_screen(40, 6);
        cpu.run(false, true);

        crt_screen_to_string(cpu.crt)
    }
}
