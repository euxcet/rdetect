use std::fs;

pub fn create_image_folder(name: &String) -> String {
    let folder = String::from("E:\\rust\\data\\") + name + "\\image";
    fs::create_dir_all(&folder).unwrap_or_else(|why| {
        println!("Error {:?}", why.kind());
    });
    folder
}

pub fn create_xml_folder(name: &String) -> String {
    let folder = String::from("E:\\rust\\data\\") + name + "\\xml";
    fs::create_dir_all(&folder).unwrap_or_else(|why| {
        println!("Error {:?}", why.kind());
    });
    folder
}