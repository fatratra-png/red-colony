#[derive(Debug)]
pub enum Evenement {
    TempeteDeSable,
    DecouverteDeGlace,
    PanneModule(String),
    JourCalme,
}

pub fn tirer_evenement() -> Evenement {
    let tirage: u32 = rand::random_range(0..10);
    match tirage {
        0 => Evenement::TempeteDeSable,
        1 => Evenement::DecouverteDeGlace,
        2 => Evenement::PanneModule("Generateur".to_string()),
        _ => Evenement::JourCalme,
    }
}
