 ///驼峰转换成下划线
 pub fn to_snake_name(name: &str) -> String {
    let chs = name.chars();
    let mut new_name = String::new();
    let mut index = 0;
    for x in chs {
        if index != 0 && x.is_uppercase() {
           new_name.push_str("_");
        } 
        new_name.push_str(x.to_lowercase().to_string().as_str());
        index += 1;
    }
    return new_name;
}