use rand::RngExt;
use rand::rngs::ThreadRng;

pub trait DiceRoller {
    fn roll_dice(&mut self) -> [u32; 4];
}

pub struct FairDiceRoller {
    rng: ThreadRng,
}

impl FairDiceRoller {
    fn new() -> FairDiceRoller {
        let rng = rand::rng();
        FairDiceRoller { rng }
    }
}

impl DiceRoller for FairDiceRoller {
    fn roll_dice(&mut self) -> [u32; 4] {
        let mut return_values = [0; 4];
        for value in return_values.iter_mut() {
            *value = self.rng.random_range(1..7)
        }
        return_values
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::{DiceRoller, FairDiceRoller};

    #[test]
    fn fair_dice_roller_returns_expected_values() {
        let mut dice_roller = FairDiceRoller::new();

        for _i in 0..10000 {
            let value = dice_roller.roll_dice();
            for v in value.iter() {
                assert!(*v >= 1);
                assert!(*v <= 6);
            }
        }
    }
}
