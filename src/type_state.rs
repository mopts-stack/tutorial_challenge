struct Employee<State> {
    name: String,
    state: State,
}

impl<State> Employee<State> {
    fn transition<NextState>(self, state: NextState) -> Employee<NextState> {
        Employee {
            name: self.name,
            state: state,
        }
    }
}

struct Agreement;
struct Signature;
struct Training;
struct FailedTraining {
    score: u8,
}
struct OnboardingComplete {
    score: u8,
}

// implementation for emplyee when it's in the Agreement state
// Agreement --> Signature
// only this state has new so this is the starting point
impl Employee<Agreement> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            state: Agreement,
        }
    }
    fn read_agreement(self) -> Employee<Signature> {
        self.transition(Signature)
    }
}

// implementation for employee when it's in the Signature
// Signature --> Training
impl Employee<Signature> {
    fn sign(self) -> Employee<Training> {
        self.transition(Training)
    }
}

// implementation for employee when it's in the Training
// Training --> OnboardingComplete or FailedTraining
impl Employee<Training> {
    fn train(self, score: u8) -> Result<Employee<OnboardingComplete>, Employee<FailedTraining>> {
        if score >= 7 {
            Ok(self.transition(OnboardingComplete { score }))
        } else {
            Err(self.transition(FailedTraining { score }))
        }
    }
}

pub fn start() {
    let employee = Employee::new("Navid");
    // Agreement -> Signature -> Traning -> FailedTraining
    let onboarded = employee.read_agreement().sign().train(6);

    match onboarded {
        Ok(emp) => println!(
            "{}: onboarding complete, score: {}",
            emp.name, emp.state.score
        ),
        Err(emp) => println!("{}: training failed, score: {}", emp.name, emp.state.score),
    }

    let employee = Employee::new("Navid");
    // Agreement -> Signature -> Traning -> OnBoardingComplete
    let onboarded = employee.read_agreement().sign().train(8);

    match onboarded {
        Ok(emp) => println!(
            "{}: onboarding complete, score: {}",
            emp.name, emp.state.score
        ),
        Err(emp) => println!("{}: training failed, score: {}", emp.name, emp.state.score),
    }
}
