use yaml_rust::Yaml;
use crate::yaml_descent;

pub fn get_description<'a>(doc: &'a Yaml, child: &'a Yaml) -> &'a str {
    let h = child.as_hash().unwrap();
    let description_key = Yaml::String(String::from("description")) ; // todo move to metadata or other persistent struct
    let parent_key = Yaml::String(String::from("parent")) ;

    let keyvecgtor = h.keys().collect::<Vec<&Yaml>>();
    
    if h.contains_key(&parent_key) {
        let parent_path = h[&parent_key].as_str().unwrap();
        let (parent, trailing, count) = yaml_descent::yaml_descent(doc, parent_path);
        return get_description(doc, parent);
    }

    if !h.contains_key(&description_key) {
        return "";
    }
    return h[&description_key].as_str().unwrap();
}