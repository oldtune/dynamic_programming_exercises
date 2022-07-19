#[allow(dead_code, unused)]
#[cfg(test)]
mod tests {
    use crate::tower_of_hanoi::{tower_of_hanoi_solve, Plate, Rod};

    #[test]
    fn tower_of_hanoi_works() {
        let mut first_rod = Rod::new(
            "First",
            vec![
                Plate::new(1, "First"),
                Plate::new(2, "Second"),
                Plate::new(3, "Third"),
            ],
        );

        let mut second_rod = Rod::new("Second", vec![]);

        let mut third_rod = Rod::new("Third", vec![]);

        tower_of_hanoi_solve(3, &mut first_rod, &mut third_rod, &mut second_rod);

        assert_eq!(3, third_rod.plates().count());

        for (i, plate) in third_rod.plates().enumerate() {
            assert_eq!(i as u8 + 1, plate.size());
        }
    }
}

mod tower_of_hanoi {
    use std::{collections::VecDeque, fmt::Display};

    pub struct Rod {
        name: String,
        plates: VecDeque<Plate>,
    }

    impl Display for Rod {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for plate in self.plates.iter().rev() {
                println!("disk {}, {}", plate.name, plate.size);
            }

            Ok(())
        }
    }

    impl Rod {
        fn can_put(&mut self, plate: &Plate) -> bool {
            match self.plates.pop_front() {
                Some(p) => p.size > plate.size,
                None => true,
            }
        }

        pub fn put(&mut self, plate: Plate) {
            self.plates.push_front(plate);
        }

        pub fn take_and_put_on_another_rod(&mut self, another_rod: &mut Rod) {
            match self.plates.pop_front() {
                Some(plate) => another_rod.put(plate),
                None => (),
            }
        }

        pub fn plates(&self) -> impl Iterator<Item = &Plate> {
            self.plates.iter()
        }

        pub fn new(name: &str, plates: Vec<Plate>) -> Self {
            Rod {
                name: name.to_string(),
                plates: plates.into(),
            }
        }
    }

    pub struct Plate {
        size: u8,
        name: String,
    }

    impl Plate {
        pub fn size(&self) -> u8 {
            self.size
        }

        pub fn new(size: u8, name: &str) -> Self {
            Plate {
                size: size,
                name: name.to_string(),
            }
        }
    }

    pub fn tower_of_hanoi_solve(plates: u8, from: &mut Rod, to: &mut Rod, helper: &mut Rod) {
        if plates == 1 {
            from.take_and_put_on_another_rod(to);
            return;
        }

        tower_of_hanoi_solve(plates - 1, from, helper, to);

        from.take_and_put_on_another_rod(to);

        tower_of_hanoi_solve(plates - 1, helper, to, from);
    }
}
