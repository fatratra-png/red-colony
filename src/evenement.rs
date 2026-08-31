use rand::Rng;

#[derive(Debug)]
pub enum Evenement {
    TempeteDeSable,
    DecouverteDeGlace,
    PanneModule(String),
    JourCalme,
}

pub fn tirer_evenement() -> Evenement {
    let mut rng = rand::thread_rng();
    let tirage = rng.gen_range(0..10);
    match tirage {
        0 => Evenement::TempeteDeSable,
        1 => Evenement::DecouverteDeGlace,
        2 => Evenement::PanneModule("Generateur".to_string()),
        _ => Evenement::JourCalme,
    }
}
