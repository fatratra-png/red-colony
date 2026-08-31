#[derive(Debug, Clone)]
pub enum Metier {
    Botaniste,
    Ingenieur,
    Medecin,
}

#[derive(Debug)]
pub struct Colon {
    pub nom: String,
    pub sante: i32,
    pub metier: Metier,
}

impl Colon {
    pub fn nouveau(nom: &str, metier: Metier) -> Colon {
        Colon {
            nom: nom.to_string(),
            sante: 100,
            metier,
        }
    }

    pub fn est_vivant(&self) -> bool {
        self.sante > 0
    }
}
