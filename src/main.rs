mod colon;
mod colonie;
mod evenement;
mod module;
mod rapport;
mod ressources;

use colon::{Colon, Metier};
use colonie::Colonie;
use module::{Module, TypeModule};
use rapport::Rapport;
use std::fs;

fn main() {
    let mut colonie = Colonie::nouvelle("Colonie Rouge");

    colonie.colons.push(Colon::nouveau("Amara", Metier::Botaniste));
    colonie.colons.push(Colon::nouveau("Théo", Metier::Ingenieur));
    colonie.colons.push(Colon::nouveau("Nadia", Metier::Medecin));

    colonie
        .modules
        .push(Module::nouveau("Serre Alpha", TypeModule::Serre));
    colonie
        .modules
        .push(Module::nouveau("Générateur Solaire", TypeModule::Generateur));
    colonie
        .modules
        .push(Module::nouveau("Réservoir Nord", TypeModule::Reservoir));

    let nb_jours = 30;

    for _ in 0..nb_jours {
        match colonie.simuler_jour() {
            Ok(()) => {
                println!("{}", colonie.generer_rapport());
            }
            Err(message) => {
                println!("{}", message);
                break;
            }
        }
    }

    let rapport_final = colonie.generer_rapport();
    fs::write("rapport_final.txt", rapport_final).expect("Impossible d'ecrire le fichier de rapport");
    println!("Rapport final sauvegarde dans rapport_final.txt");
}
