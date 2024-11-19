use crate::chemistry::{ChemistryDefs, ChemistryName};
use std::collections::HashMap;
use lazy_static::lazy_static;
use itertools::Itertools;


const CHEMISTRY_DEFS_JSON_STR: &str = std::include_str!("chemistry_defs.json");

fn read_chemistry_defs_json() ->Result<String, std::io::Error> {
    let lib_bin_exe = env::current_exe()?;
    let lib_bin_dir = lib_bin_exe.parent()
        .ok_or_else(|| anyhow!("Could not find parent directory of binary")
        )?;
    let libdir = lib_bin_dir.parent()
        .ok_or_else(|| anyhow!("Could not find grandparent directory of binary")
        )?;
    Ok(
        std::fs::read_to_string(
            libdir.join("chemistry_defs.json")
        )?
    )
}

lazy_static! {
    static ref CHEMISTRY_DEF_MAP: HashMap<ChemistryName, ChemistryDefs> = {
        if let Ok(json_str) = read_chemistry_defs_json() {
            serde_json::from_str(&json_str).unwrap()
        } else {
            serde_json::from_str(CHEMISTRY_DEFS_JSON_STR).unwrap()
        }
    }
}

pub fn known_chemistry_defs() -> &'static HashMap<ChemistryName, ChemistryDefs> {
    &CHEMISTRY_DEF_MAP
}

pub fn get_chemistry_defs(name: ChemistryName) -> Option<&'static ChemistryDefs>  {
    let maybe_def = CHEMISTRY_DEF_MAP.get(&name);
    if let Some(def) = maybe_def {
        assert_eq!(def.name, name);
    }
}

pub fn normalize_chemistry_def(mut def: ChemistryDefs) -> Option<ChemistryDefs> {
    CHEMISTRY_DEF_MAP
        .values()
        .filter(|known_df|{
            def.name = known_df.name;
            def.description = known_df.description.clone();
            def == **known_df
        })
        .exactly_one()
        .ok()
        .cloned()
}



