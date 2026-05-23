use std::fs;

use crate::lineage::CognitionLineage;

pub fn persist_lineage(lineage: &CognitionLineage) {
    fs::create_dir_all("lineage").unwrap();

    let path = format!("lineage/{}.json", lineage.lineage_id);

    let serialized = serde_json::to_string_pretty(lineage).unwrap();

    fs::write(path, serialized).unwrap();

    println!("[LINEAGE] persisted {}", lineage.lineage_id);
}
