#[derive(Debug, Clone)]
pub enum TypeModule {
    Serre,
    Generateur,
    Reservoir,
    Labo,
}

#[derive(Debug)]
pub struct Module {
    pub nom: String,
    pub type_module: TypeModule,
}

impl Module {
    pub fn nouveau(nom: &str, type_module: TypeModule) -> Module {
        Module {
            nom: nom.to_string(),
            type_module,
        }
    }

    pub fn produire(&self) -> (&str, i32) {
        match self.type_module {
            TypeModule::Serre => ("nourriture", 15),
            TypeModule::Generateur => ("energie", 20),
            TypeModule::Reservoir => ("eau", 10),
            TypeModule::Labo => ("energie", -5),
        }
    }
}
