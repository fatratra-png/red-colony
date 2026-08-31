pub trait Rapport {
    fn generer_rapport(&self) -> String;
}

use crate::colonie::Colonie;

impl Rapport for Colonie {
    fn generer_rapport(&self) -> String {
        format!(
            "=== Jour {} — Colonie {} ===\nOxygene : {}\nEau : {}\nNourriture : {}\nEnergie : {}\nColons vivants : {}\n",
            self.jour,
            self.nom,
            self.ressources.oxygene,
            self.ressources.eau,
            self.ressources.nourriture,
            self.ressources.energie,
            self.colons.iter().filter(|c| c.est_vivant()).count()
        )
    }
}
