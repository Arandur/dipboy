#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum ProvinceKind {
    LAND,
    COAST,
    SEA
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Province {
    name: &'static str,
    kind: ProvinceKind,
    sc: bool
}

pub static PROVINCES: [Province; 74] = [
    Province { name: "Bohemia", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Budapest", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Galicia", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Triste", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Tyrolia", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Vienna", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Clyde", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Edinburgh", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Liverpool", kind: ProvinceKind::LAND, sc: true },
    Province { name: "London", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Wales", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Yorkshire", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Brest", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Burgundy", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Gascony", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Marseilles", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Paris", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Picardy", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Berlin", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Kiel", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Munich", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Prussia", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Ruhr", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Silesia", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Apulia", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Naples", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Piedmont", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Rome", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Tuscany", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Venice", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Livonia", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Moscow", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Sevastopol", kind: ProvinceKind::LAND, sc: true },
    Province { name: "St. Petersburg", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Ukraine", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Warsaw", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Ankara", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Armenia", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Constantinople", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Smyrna", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Syria", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Albania", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Belgium", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Bulgaria", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Finland", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Greece", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Holland", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Norway", kind: ProvinceKind::LAND, sc: true },
    Province { name: "North_Africa", kind: ProvinceKind::LAND, sc: false },
    Province { name: "Portugal", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Rumania", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Serbia", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Spain", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Sweden", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Tunis", kind: ProvinceKind::LAND, sc: true },
    Province { name: "Adriatic Sea", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Aegean Sea", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Baltic Sea", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Barents Sea", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Black Sea", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Eastern Mediterranean", kind: ProvinceKind::SEA, sc: false },
    Province { name: "English Channel", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Gulf of Bothnia", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Gulf of Lyon", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Helgoland Bight", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Ionian Sea", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Irish Sea", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Mid Atlantic Ocean", kind: ProvinceKind::SEA, sc: false },
    Province { name: "North Atlantic Ocean", kind: ProvinceKind::SEA, sc: false },
    Province { name: "North Sea", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Norwegian Sea", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Skagerrak", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Tyrrhenian Sea", kind: ProvinceKind::SEA, sc: false },
    Province { name: "Western Mediterranean", kind: ProvinceKind::SEA, sc: false },
];

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Hold {
    unit: &'static Province
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Move {
    unit: &'static Province,
    to: &'static Province
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Support {
    unit: &'static Province,
    from: &'static Province,
    to: &'static Province
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Convoy {
    unit: &'static Province,
    from: &'static Province,
    to: &'static Province
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Order {
    Hold(Hold),
    Move(Move),
    Support(Support),
    Convoy(Convoy)
}

impl Order {
    pub fn unit(&self) -> &'static Province {
        match *self {
            Order::Hold(h) => h.unit,
            Order::Move(m) => m.unit,
            Order::Support(s) => s.unit,
            Order::Convoy(c) => c.unit
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_unit() {
        let order = Order::Hold(Hold { unit: &PROVINCES[0] });

        assert_eq!(order.unit(), &PROVINCES[0]);
    }
}
