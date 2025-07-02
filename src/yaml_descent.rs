use yaml_rust::Yaml;
use regex::Regex;
use yaml_rust::yaml::Hash;

const  WHOLE_MATCH: usize = 0 ;
const KEY_MATCH: usize = 1 ;
const PERIOD_MATCH: usize = 2 ;
const INDEX_MATCH: usize = 3 ;
const ARRAY_MATCH: usize = 4;

pub fn yaml_descent<'a, 'h>(tree: &'a Yaml, path: &str) -> (&'a Yaml, String, usize) {
    let re = Regex::new(r"([^.\[\]\\]+)(\.)?|(?:\\?\[(\d+)\\?\]?|(\\?\[$))?").unwrap() ;
    let mut current: &Yaml = tree ;
    let mut depth = 0 ;
    let mut current_key = "" ;
    let mut prev_current = current ;
    for captures in re.captures_iter(path) {
        if captures.get(KEY_MATCH).is_some() {
            let key = captures.get(KEY_MATCH).unwrap().as_str();
            if !matches!(current, Yaml::Hash(_)) {
                panic!("Attempting to use key on non-hash node");
            }
            prev_current = current ;
            current = &current[key] ;
            if current.is_badvalue() {
                return (prev_current, key.to_string(), depth)
            }
            current_key = key ;
            continue ;
        }
        if captures.get(INDEX_MATCH).is_some() {
            let index = captures.get(INDEX_MATCH).unwrap().as_str().parse::<usize>().unwrap();
            if !matches!(current, Yaml::Array(_)) {
                panic!("Attempting to use array index on non-array node");
            }
            current = &current[index] ;
            if current.is_badvalue() {
                panic!("array bounds exceeded");
            }
        }
        depth += 1 ;
    } // for
    (current, String::from(current_key), depth)  //
}