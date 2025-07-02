#[cfg(test)]
pub mod u_tests {
    use crate::strwriter::StrWriter;
    use crate::write_completions;
    use crate::write_completions::write_completions;
    use crate::yaml_descent::yaml_descent;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Read};
    use yaml_rust::{Yaml, YamlLoader};
    use crate::get_description::get_description;

    const SOURCE1: &str = r#"---
field1:
    field1a: value1
    field1b: value2
    
field2: foo
   
"# ;
    
    const SOURCE2: &str = r#"---
field:
    fielda: value1
    fieldb: value2
"# ;
    fn get_yaml()  -> Yaml {
        let mut contents = String::new();
        let file = File::open("dumper.yaml").expect("Unable to open the file");
        let mut buf_reader = BufReader::new(file) ;
        buf_reader.read_to_string(&mut contents).expect("Unable to read the file");

        YamlLoader::load_from_str(&contents).expect("load/parse failed")[0].clone()
    }

    fn input_output_check(input: &str, output: &str) {
        let tree = get_yaml();
        let mut result_buffer = StrWriter::new() ;
        write_completions::write_completions(&mut result_buffer, &tree, &input, &false).expect("write failed") ;
        let result_str = result_buffer.to_string().expect("write failed") ;
        assert_eq!(result_str, output);
    }

    #[test]
    fn test_empty() {
        let y = &YamlLoader::load_from_str(SOURCE1).unwrap()[0] ;
        let mut output = BufWriter::new(Vec::new());
        write_completions::write_completions(&mut output, &y, "", &false).expect("write failed") ;
        let s = String::from_utf8(output.into_inner().unwrap()).unwrap();
        assert_eq!(s, "field1\nfield2\n");
    }
    
    #[test]
    fn test_one_path() {
        let y = &YamlLoader::load_from_str(SOURCE1).unwrap()[0] ;
        let mut output = StrWriter::new() ;
        write_completions(&mut output, &y, "f", &false).expect("write failed") ;
        let s = output.to_string().expect("write failed") ;
        assert_eq!(s, "field1\nfield2\n");
    }

    #[test]
    fn test_two_paths() {
        let source = r#"---
field1: value1
field2: value2        
"# ;
        let y = &YamlLoader::load_from_str(SOURCE1).unwrap()[0] ;
        let mut output = BufWriter::new(Vec::new());
        write_completions::write_completions(&mut output, &y, "f", &false).expect("write failed") ;
        let s = String::from_utf8(output.into_inner().unwrap()).unwrap();
        assert_eq!(s, "field1\nfield2\n");
    }
    
    #[test]
    fn test_two_paths_2() {
        let y = &YamlLoader::load_from_str(SOURCE1).unwrap()[0] ;
        let mut output = BufWriter::new(Vec::new());
        write_completions::write_completions(&mut output, &y, "f", &false).expect("write failed") ;
        let s = String::from_utf8(output.into_inner().unwrap()).unwrap();
        assert_eq!(s, "field1\nfield2\n");
    }
    
    #[test]
    fn test_array1() {
        input_output_check("array", "array[0]\narray[1]\narray[2]\n") ;
    }
    #[test]
    fn test_array2() {
        input_output_check("array[2][", "array[2][0]\narray[2][1]\narray[2][2]\n") ;
    }

    #[test]
    fn test_array3() {
        input_output_check("array[", "array[0]\narray[1]\narray[2]\n") ;
    }
    #[test]
    fn test_field_terminator() {
        input_output_check("level1.", "level1.level2\nlevel1.level2a\nlevel1.level2b\n") ;
    }
    
    #[test]
    fn test_level_drop() {
        input_output_check("level1.level2", "level1.level2\nlevel1.level2a\nlevel1.level2b\n") ;
        input_output_check("level1.level2a", "level1.level2a\n") ;
    }
    
    #[test]
    fn test_gpio() {
        input_output_check("G", "GPIO.pins\nGPIO.words\n") ;
    }
    
    #[test]
    fn test_gpio_p() {
        input_output_check("GPIO.p", "GPIO.pins[0]\nGPIO.pins[1]\n") ;
    }
    #[test]
    fn test_gpio_pin0() {
        input_output_check("GPIO.pins[0].", "GPIO.pins[0].clear\nGPIO.pins[0].function\nGPIO.pins[0].level\nGPIO.pins[0].set\n") ;
    }

    static ysource: &str = "---
GPIO:
    pins:
      - set:
          offset: 0x10000000
      - clear:
          offset: 0x10000004
    " ;


    #[test]
    fn test_descent() {
        let docs = YamlLoader::load_from_str(ysource).unwrap();
        let doc = &docs[0];

        let result = yaml_descent(doc, "GPIO.pins[0].s") ;
        // assert_eq!(result, "et") ;

    }
    
    #[test]
    fn test_descending() {

        input_output_check("ulev", "ulevel.level1.level2.level3\n") ;
       
        return ;
    }
    
    #[test]
    fn test_get_description() {
        
        let y = get_yaml() ;
        let (child, _, _depth) = yaml_descent(&y, "test_description.description_target") ;
        let description = get_description(&y, child) ;
        assert_eq!(description, "this is a description") ;
        return ;
    }
    
}