// 
// SPDX-License-Identifier: MIT
// 
// Copyright (c) 2025 Andrew Ellis Page
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
// 
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