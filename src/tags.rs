extern crate gtk;


use std::vec::Vec;

struct TagManager 
{
    tags: Vec<String>,
}

impl TagManager
{
    pub fn new() -> TagManager 
    {
        TagManager {
            tags: Vec::new(),
        }
    }

    pub fn add_tag(&mut self, tag: String) 
    {
        self.tags.push(tag);
    }

    pub fn clear_tags(&mut self) 
    {
        self.tags.clear();
    }

    pub fn get_tags(&self) -> &Vec<String>
    {
        &self.tags
    }

    pub fn display_in_list_box(&self, list: gtk::ListBox) 
    {
        
    }
}
