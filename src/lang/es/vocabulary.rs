use phf::{phf_set, Set};

pub static INSIGNIFICANT: Set<&'static str> = phf_set! {
    "alors", "bien", "c'est", "encore", "ensuite", "y", "euh", "heu", "ha", "ah", "hu", "hum", "moins", "ok", "oui", "plus", "puis", "voilà"
};
