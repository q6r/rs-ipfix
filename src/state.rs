use parser;
use rustc_hash::FxHashMap as HashMap;

#[derive(Debug, new)]
pub struct State {
    #[new(default)]
    templates: HashMap<u16, parser::Template>,
    #[new(default)]
    options_templates: HashMap<u16, parser::OptionsTemplate>,
}

impl State {
    pub fn add_template(&mut self, id: u16, template: parser::Template) {
        self.templates.insert(id, template);
    }

    pub fn add_options_template(&mut self, id: u16, options_template: parser::OptionsTemplate) {
        self.options_templates.insert(id, options_template);
    }

    pub fn get_template(&self, id: &u16) -> Option<&parser::Template> {
        self.templates.get(id)
    }

    pub fn get_templates(&self) -> &HashMap<u16, parser::Template> {
        &self.templates
    }

    pub fn get_options_templates(&self) -> &HashMap<u16, parser::OptionsTemplate> {
        &self.options_templates
    }

    pub fn get_options_template(&self, id: &u16) -> Option<&parser::OptionsTemplate> {
        self.options_templates.get(id)
    }

    pub fn templates_len(&self) -> usize {
        self.templates.len()
    }

    pub fn options_templates_len(&self) -> usize {
        self.options_templates.len()
    }

    pub fn len(&self) -> usize {
        self.templates_len() + self.options_templates_len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
