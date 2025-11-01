use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct JsonSpell {
    spell_id: String,
    name: String,
    #[serde(default)]
    is_attribute_bolt: bool,
}

#[derive(Debug, Deserialize)]
struct SpellsData {
    spells: Vec<JsonSpell>,
}

fn main() {
    let spells_json = include_str!("../spells.json");
    let spells_data: SpellsData = serde_json::from_str(spells_json).unwrap();
    
    println!("Total spells in JSON: {}", spells_data.spells.len());
    
    let attribute_bolts: Vec<&JsonSpell> = spells_data.spells.iter()
        .filter(|s| s.is_attribute_bolt)
        .collect();
    
    println!("\nAttribute bolts found: {}", attribute_bolts.len());
    for bolt in attribute_bolts {
        println!("  - {} ({})", bolt.spell_id, bolt.name);
    }
}
