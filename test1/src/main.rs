use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
struct FqRecord {
    #[serde(with = "serde_bytes")]
    head: Vec<u8>,
    #[serde(with = "serde_bytes")]
    seq: Vec<u8>,
    #[serde(with = "serde_bytes")]
    qual: Vec<u8>,
}

impl FqRecord {
    fn to_string(&self) -> String {
        let head = String::from_utf8_lossy(&self.head);
        let seq = String::from_utf8_lossy(&self.seq);
        let qual = String::from_utf8_lossy(&self.qual);

        format!(
            "@{}\n{}\n+\n{}\n",
            head, seq, qual
        )
    }
}

fn main() {
    let record = FqRecord {
        head: b"@SEQ_ID".to_vec(),
        seq: b"GATTTGGGGTTCAAAGCAGTATCGATCAAATAGTAAATCCATTTGTTCAACTTGATGTGACGAGCCTCAGTATTT".to_vec(),
        qual: b"IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII".to_vec(),
    };

    let json = serde_json::to_string(&record).unwrap();
    println!("Serialized FqRecord: {}", json);

    let deserialized_record: FqRecord = serde_json::from_str(&json).unwrap();
    println!("Deserialized FqRecord: {:?}", deserialized_record);

    println!("{}", deserialized_record.to_string());
    

    let mut rg_items = std::collections::HashMap::new();
    for i in 1..100 {
        let name = format!("gemgroup{:03}", i);
        rg_items.insert(name.clone(), (name, 0));
    }
    println!("{:?}", rg_items)
}
