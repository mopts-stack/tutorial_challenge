#[derive(Clone, Copy, Debug)]
struct Volume(usize);

trait ReagentContainer {
    fn max_volume(&self) -> Volume;
    fn current_volume(&self) -> Volume;
}

macro_rules! impl_reagent_container {
    ($container:ty : $volume:literal) => {
        impl ReagentContainer for $container {
            fn max_volume(&self) -> Volume {
                Volume($volume)
            }
            fn current_volume(&self) -> Volume {
                self.current_volume
            }
        }
    };
}

#[derive(Clone, Copy, Debug)]
struct TallFlask {
    current_volume: Volume,
}

#[derive(Clone, Copy, Debug)]
struct TestTube {
    current_volume: Volume,
}

#[derive(Clone, Copy, Debug)]
struct Pipette {
    current_volume: Volume,
}

// impl ReagentContainer for Pipette {
//     fn max_volume(&self) -> Volume {
//         Volume(4)
//     }

//     fn current_volume(&self) -> Volume {
//         self.current_volume
//     }
// }

#[derive(Clone, Copy, Debug)]
struct OtherTube {
    current_volume: Volume,
    max_volume: Volume,
}

impl ReagentContainer for OtherTube {
    fn max_volume(&self) -> Volume {
        self.max_volume
    }

    fn current_volume(&self) -> Volume {
        self.current_volume
    }
}

impl_reagent_container!(TallFlask: 32);
impl_reagent_container!(TestTube: 10);
impl_reagent_container!(Pipette: 4);

pub fn start() {
    let tube = TestTube {
        current_volume: Volume(0),
    };

    dbg!(tube.max_volume());
}
