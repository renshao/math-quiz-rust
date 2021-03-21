use rand::prelude::*;
use rand_seeder::Seeder;
use rand_pcg::Pcg64;


struct Question {
    number1: u32,
    operator: char,
    number2: u32,
    answer: u32
}

impl PartialEq for Question {
    fn eq(&self, other: &Self) -> bool {
        self.number1 == other.number1 && self.number2 == other.number2 && self.operator == other.operator
    }
}
impl Eq for Question {}


fn main() {
    let mut rng: Pcg64 = Seeder::from("seed to make consistent questions between multiple runs").make_rng();
    let mut questions = Vec::new();
    
    for i in 0..30 {
        let mut q = generate_question(&mut rng);

        while questions.contains(&q) {
            q = generate_question(&mut rng);
        }
        questions.push(q);
        
        println!("{} {} {} = {}", questions[i].number1, questions[i].operator, questions[i].number2, questions[i].answer);
    }
    
}

fn generate_question(rng: &mut Pcg64) -> Question {
    let possible_operators = ['×', '÷'];
    let operator = *possible_operators.choose(rng).unwrap();
    
    let mut number1 = rng.gen_range(2..10);
    let number2 = rng.gen_range(2..10);
    let mut answer = number1 * number2;

    if operator == '÷' {
        std::mem::swap(&mut number1, &mut answer);
    }

    return Question {
        number1: number1,
        operator: operator,
        number2: number2,
        answer: answer
    };
}
