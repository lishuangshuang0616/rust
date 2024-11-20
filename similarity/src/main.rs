use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(Debug, Clone)]
struct BcInfo {
    bead: u64,
    m280: u64,
    cnt: i64,
}

#[derive(Debug, Clone)]
struct BeadInfo {
    b: i32,
    e: i32,
    bead: u64,
}

#[derive(Debug, Clone)]
struct SeqData {
    infos: Vec<BcInfo>,
    idx_arr: Vec<i32>,
}

#[derive(Debug, Clone)]
struct Droplet {
    infos: Vec<BcInfo>,
    beads: Vec<BeadInfo>,
}

#[derive(Debug, Clone)]
struct BcSim {
    m280: u64,
    val: i32,
    idx: i32,
    rank: f64,
}

#[derive(Debug, Clone)]
struct SimRes {
    bead: [u64; 2],
    similarity: f64,
}

impl Hash for SimRes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bead[0].hash(state);
        self.bead[1].hash(state);
    }
}

impl PartialEq for SimRes {
    fn eq(&self, other: &Self) -> bool {
        (self.bead[0] == other.bead[0] && self.bead[1] == other.bead[1]) ||
        (self.bead[0] == other.bead[1] && self.bead[1] == other.bead[0])
    }
}

impl Eq for SimRes {}

#[derive(Debug, Clone)]
struct BeadMap {
    map: HashMap<u64, f64>,
}

#[derive(Debug, Clone)]
struct SimBox {
    map: HashMap<u64, BeadMap>,
}

fn sim_res_hash(res: &SimRes) -> u64 {
    res.bead[0] | res.bead[1]
}

fn sim_res_equal(a: &SimRes, b: &SimRes) -> bool {
    (a.bead[0] == b.bead[0] && a.bead[1] == b.bead[1]) ||
    (a.bead[0] == b.bead[1] && a.bead[1] == b.bead[0])
}

fn bead_map_init() -> BeadMap {
    BeadMap {
        map: HashMap::new(),
    }
}

fn bead_map_copy(dst: &mut BeadMap, src: &BeadMap) {
    dst.map.clone_from(&src.map);
}

fn bead_map_clear(map: &mut BeadMap) {
    map.map.clear();
}

fn bead_map_free(map: BeadMap) {
    drop(map);
}

fn box_key_dump(out: &mut dyn Write, key: &u64) -> std::io::Result<()> {
    write!(out, "{:X}-{}:\n", key, key)
}

fn bead_map_key_dump(out: &mut dyn Write, key: &u64) -> std::io::Result<()> {
    write!(out, "  {:X}", key)
}

fn bead_map_val_dump(out: &mut dyn Write, val: &f64) -> std::io::Result<()> {
    write!(out, " - {:.6}", val)
}

fn box_val_dump(out: &mut dyn Write, val: &BeadMap) -> std::io::Result<()> {
    for (k, v) in &val.map {
        bead_map_key_dump(out, k)?;
        bead_map_val_dump(out, v)?;
        writeln!(out)?;
    }
    Ok(())
}

fn seq_data_init() -> SeqData {
    SeqData {
        infos: Vec::new(),
        idx_arr: Vec::new(),
    }
}

fn dropset_init() -> Droplet {
    Droplet {
        infos: Vec::new(),
        beads: Vec::new(),
    }
}

fn add1seq2wl(wl: &mut HashMap<u32, u32>, cs: u32, len: i32) {
    for i in 0..len {
        let mask = !(0x7 << (3 * i));
        let cs2 = (cs & mask) | (0x0 << (3 * i)); // A
        wl.insert(cs2, cs);
        let cs2 = (cs & mask) | (0x1 << (3 * i)); // C
        wl.insert(cs2, cs);
        let cs2 = (cs & mask) | (0x2 << (3 * i)); // G
        wl.insert(cs2, cs);
        let cs2 = (cs & mask) | (0x3 << (3 * i)); // T
        wl.insert(cs2, cs);
    }
}

