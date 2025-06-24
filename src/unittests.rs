#[cfg(test)]
pub mod u_tests {
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Read};
    use yaml_rust::{Yaml, YamlLoader};
    use crate::write_completions;
    static Source1 : &str = r#"---
field1:
    field1a: value1
    field1b: value2
    
field2: foo
   
"# ;
    
    static Source2 : &str = r#"---
field:
    fielda: value1
    fieldb: value2
"# ;




    #[test]
    fn test_empty() {
        let y = &YamlLoader::load_from_str(Source1).expect("load/parse failed")[0] ;
        let mut output = BufWriter::new(Vec::new());
        write_completions::write_completions(&mut output, y, "").expect("write failed") ;
        let s = String::from_utf8(output.into_inner().unwrap()).unwrap();
        assert_eq!(s, "field1\nfield2\n");
    }
    
    #[test]
    fn test_one_path() {
        let y = &YamlLoader::load_from_str(Source1).expect("load/parse failed")[0] ;
        let mut output = BufWriter::new(Vec::new());
        write_completions::write_completions(&mut output, y, "f").expect("write failed") ;
        let s = String::from_utf8(output.into_inner().unwrap()).unwrap();
        assert_eq!(s, "field1\nfield2\n");
    }

    #[test]
    fn test_two_paths() {
        let source = r#"---
field1: value1
field2: value2        
"# ;
        let y = &YamlLoader::load_from_str(source).expect("load/parse failed")[0] ;
        let mut output = BufWriter::new(Vec::new());
        write_completions::write_completions(&mut output, y, "f").expect("write failed") ;
        let s = String::from_utf8(output.into_inner().unwrap()).unwrap();
        assert_eq!(s, "field1\nfield2\n");
    }
    
    #[test]
    fn test_two_paths_2() {
        let y = &YamlLoader::load_from_str(Source2).expect("load/parse failed")[0] ;
        let mut output = BufWriter::new(Vec::new());
        write_completions::write_completions(&mut output, y, "f").expect("write failed") ;
        let s = String::from_utf8(output.into_inner().unwrap()).unwrap();
        assert_eq!(s, "field.fielda\nfield.fieldb\n");
    }
    
    fn get_yaml() -> Yaml {
        let mut contents = String::new();
        let file = File::open("dumper.yaml").expect("Unable to open the file");
        let mut buf_reader = BufReader::new(file) ;
        buf_reader.read_to_string(&mut contents).expect("Unable to read the file");

        YamlLoader::load_from_str(&contents).expect("load/parse failed")[0].clone()
    }

    #[test]
    fn test_array1() {
        let current = get_yaml();
        let mut output = BufWriter::new(Vec::new());
        write_completions::write_completions(&mut output, &current, "array").expect("write failed") ;
        let s = String::from_utf8(output.into_inner().unwrap()).unwrap();
        assert_eq!(s, "array[0]\narray[1]\narray[2]\n");
    }
    
}