use yaml_rust::Yaml;

pub fn get_description(doc: &Yaml, child: &Yaml) -> String {
    let h = child.as_hash().unwrap();
    let description_key = Yaml::String(String::from("description")) ; // todo move to metadata or other persistent struct
    let parent_key = Yaml::String(String::from("parent")) ;

    // let keyvector = h.keys().collect::<Vec<&Yaml>>();
    
    if h.contains_key(&parent_key) {
        let parent_path = h[&parent_key].as_str().unwrap();
        // let (parent, trailing, count) = yaml_descent::yaml_descent(doc, parent_path);
        todo!() ;
        //return get_description(doc, parent);
    }

    if !h.contains_key(&description_key) {
        return String::from("");
    }
    return h[&description_key].as_str().unwrap().to_string()
}