use crate::colon::Colon;
use crate::evenement::{tirer_evenement, Evenement};
use crate::module::Module;
use crate::ressources::Ressources;

pub struct Colonie {
    pub nom: String,
    pub jour: u32,
    pub colons: Vec<Colon>,
    pub modules: Vec<Module>,
    pub ressources: Ressources,
}

impl Colonie {
    pub fn nouvelle(nom: &str) -> Colonie {
        Colonie {
            nom: nom.to_string(),
            jour: 0,
            colons: Vec::new(),
            modules: Vec::new(),
            ressources: Ressources::depart(),
        }
    }

    pub fn simuler_jour(&mut self) -> Result<(), String> {
        self.jour += 1;

        for module in &self.modules {
            let (ressource, quantite) = module.produire();
            self.ressources.ajouter(ressource, quantite);
        }

        let nb_colons = self.colons.iter().filter(|c| c.est_vivant()).count() as i32;
        self.ressources.ajouter("nourriture", -nb_colons * 2);
        self.ressources.ajouter("oxygene", -nb_colons * 1);

        let evenement = tirer_evenement();
        self.appliquer_evenement(&evenement);

        if self.ressources.en_danger() {
            return Err(format!(
                "Jour {} : une ressource critique est epuisee !",
                self.jour
            ));
        }

        Ok(())
    }

    fn appliquer_evenement(&mut self, evenement: &Evenement) {
        match evenement {
            Evenement::TempeteDeSable => self.ressources.ajouter("energie", -15),
            Evenement::DecouverteDeGlace => self.ressources.ajouter("eau", 25),
            Evenement::PanneModule(nom) => {
                println!("! Panne detectee sur le module : {}", nom)
            }
            Evenement::JourCalme => {}
        }
    }
}
