use std::string::String;
use std::collections::HashSet;
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;

pub struct Metadata {
    pub root: Yaml,
    pub terminus: Yaml,
    pub ignore_fields: HashSet<String>,
    pub get_descriptions: bool,
    
    has_terminus: bool
}

impl Metadata {
    fn new(root: Yaml, terminus: Yaml, ignores: Vec<&Yaml>, get_descriptions: bool) -> Metadata {
        let h: HashSet<String> = ignores.into_iter()
            .map(|y| y.as_str().unwrap().to_string())
            .collect();
        let has_terminus = terminus.as_str().is_some();
        Metadata {
            root: root,
            terminus: terminus,
            ignore_fields: h,
            get_descriptions: get_descriptions,
            has_terminus: has_terminus,
        }
    }
    
    pub fn has_root(&self) -> bool {
        self.root.as_str().is_some()
    }
    
    pub fn has_terminus(&self) -> bool {
        self.has_terminus
    }
    pub fn has_terminal_field(&self, yaml: &Yaml) -> bool {
        self.has_terminus && match yaml {
            Yaml::Hash(h) => {
                 h.contains_key(&self.terminus)
            },
            _ => { false }
        }
    }
}

pub fn get_metadata(doc: &Yaml, get_descriptions: bool) -> Metadata {
    let metakey = Yaml::String("completion-metadata".to_string());
    let rootkeykey = Yaml::String("root".to_string());
    let terminuskey = Yaml::String("terminus".to_string());
    let ignoreskey = Yaml::String("ignore-fields".to_string());
    
    let Yaml::Hash(tree) = doc else { return Metadata::new(Yaml::BadValue, Yaml::BadValue, Vec::new(), false) };
    if !tree.contains_key(&metakey) {
        return Metadata::new(Yaml::BadValue, Yaml::BadValue, Vec::new(), false);
    }
    let &Yaml::Hash(meta) = &tree.get(&metakey).unwrap() else { panic!("Metadata must be a hash"); };

    let root = &meta[&rootkeykey];
    let terminus = if meta.contains_key(&terminuskey) {&meta[&terminuskey] } else { &Yaml::BadValue };

    let ignore_fields = match meta.get(&ignoreskey) {
        Some(Yaml::Array(arr)) => arr.iter()
            .collect(),
        _ => Vec::new(),
    };
    Metadata::new(root.clone(), terminus.clone(),
                  ignore_fields, get_descriptions)
}