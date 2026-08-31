#[derive(Debug)]
pub struct Ressources {
    pub oxygene: i32,
    pub eau: i32,
    pub nourriture: i32,
    pub energie: i32,
}

impl Ressources {
    pub fn depart() -> Ressources {
        Ressources {
            oxygene: 100,
            eau: 100,
            nourriture: 100,
            energie: 100,
        }
    }

    pub fn ajouter(&mut self, ressource: &str, quantite: i32) {
        match ressource {
            "oxygene" => self.oxygene += quantite,
            "eau" => self.eau += quantite,
            "nourriture" => self.nourriture += quantite,
            "energie" => self.energie += quantite,
            _ => {}
        }
    }

    pub fn en_danger(&self) -> bool {
        self.oxygene <= 0 || self.eau <= 0 || self.nourriture <= 0
    }
}
