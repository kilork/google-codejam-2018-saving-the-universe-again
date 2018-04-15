fn main() {
    let cases_count: usize = read_line().parse().unwrap();
    for case in 1..cases_count + 1 {
        println!("Case #{}: {}", case, min_hacks());
    }
}

fn min_hacks() -> Hack {
    let line = read_line();
    let mut words = line.split_whitespace();
    if let (Some(shield), Some(program)) = (words.next(), words.next()) {
        let shield: u32 = shield.parse().unwrap();
        let mut hackable = parse_program(program);
        hackable.hack(shield)
    } else {
        Hack::Impossible
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ProgramStatement {
    Shot,
    Charge,
}

enum Hack {
    Impossible,
    SwapsCount(u32),
}

impl std::fmt::Display for Hack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Hack::Impossible => write!(f, "IMPOSSIBLE"),
            &Hack::SwapsCount(count) => write!(f, "{}", count),
        }
    }
}

#[derive(Debug)]
struct Hackable {
    shots_count: u32,
    charges_count: u32,
    program: Vec<ProgramStatement>,
}

fn program_damage(program: &[ProgramStatement]) -> u32 {
    let mut hit = 1;
    let mut damage = 0;
    program.iter().for_each(|x| match x {
        &ProgramStatement::Shot => damage += hit,
        &ProgramStatement::Charge => hit *= 2,
    });
    damage
}

impl Hackable {
    fn damage(&self) -> u32 {
        if self.shots_count == 0 {
            return 0;
        }
        if self.charges_count == 0 {
            return self.shots_count;
        }
        program_damage(&self.program)
    }

    fn hack(&self, shield_max_damage: u32) -> Hack {
        let mut robot_damage = self.damage();
        if robot_damage < shield_max_damage {
            return Hack::SwapsCount(0);
        }
        let mut program = self.program.clone();
        let mut hacks_count = 0;
        while robot_damage > shield_max_damage {
            let mut seek_shot = program.len() - 1;
            while program[seek_shot] == ProgramStatement::Charge {
                seek_shot -= 1;
            }

            let mut seek_charge = seek_shot;
            while program[seek_charge] == ProgramStatement::Shot {
                if seek_charge == 0 {
                    return Hack::Impossible;
                }
                seek_charge -= 1;
            }

            program.swap(seek_charge, seek_charge + 1);
            hacks_count += 1;
            robot_damage = program_damage(&program);
        }
        Hack::SwapsCount(hacks_count)
    }
}

fn parse_program(program: &str) -> Hackable {
    let mut shots_count = 0;
    let mut charges_count = 0;

    let program = program
        .chars()
        .map(|x| match x {
            'S' => {
                shots_count += 1;
                ProgramStatement::Shot
            }
            'C' => {
                charges_count += 1;
                ProgramStatement::Charge
            }
            _ => panic!(),
        })
        .collect();

    Hackable {
        shots_count,
        charges_count,
        program,
    }
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
