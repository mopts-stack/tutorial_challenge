//  An airline wants to reduce the amount of lost luggage by
//  ensuring the luggage is properly tracked.
//
//  Requirements:
//  * Implement a luggage tracking system using the typestate pattern
//  * Each piece of luggage has a tracking id
//  * Luggage goes through multiple states at the airport
//   - CheckIn:          passenger gives luggage to airport
//   - OnLoading:        luggage is loaded onto correct place
//   - OffLoading:       luggage is taken off placce at destination
//   - AwaitingPickup:   luggage is at destination waiting for passenger pickup
//   - EndCustody:       luggage was picked up by passenger
//
//  Notes:
//  * Optionally use generics for each state

#[derive(Copy, Clone)]
struct LuggageId(usize);
struct Luggage(LuggageId);
struct CheckIn(LuggageId);
struct OnLoad(LuggageId);
struct OffLoad(LuggageId);
struct AwaitingPickup(LuggageId);
struct EndCustody(LuggageId);

impl Luggage {
    fn new(id: LuggageId) -> Self {
        Luggage(id)
    }

    fn check_in(self) -> CheckIn {
        CheckIn(self.0)
    }
}

impl CheckIn {
    fn onload(self) -> OnLoad {
        OnLoad(self.0)
    }
}

impl OnLoad {
    fn offload(self) -> OffLoad {
        OffLoad(self.0)
    }
}

impl OffLoad {
    fn carousel(self) -> AwaitingPickup {
        AwaitingPickup(self.0)
    }
}

impl AwaitingPickup {
    fn pickup(self) -> (Luggage, EndCustody) {
        (Luggage(self.0), EndCustody(self.0))
    }
}

pub fn start() {
    let id = LuggageId(1);
    let luggage = Luggage::new(id);
    let luggage = luggage.check_in().onload().offload().carousel();
    let (luggage, _) = luggage.pickup();

    println!("Luggage is delivered");
}
