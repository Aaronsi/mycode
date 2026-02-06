use anyhow::{Context, Result};
use minijinja::{context, Environment};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Prompt template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    /// Template name
    pub name: String,
    /// Template content
    pub content: String,
    /// Template variables
    pub variables: Vec<String>,
}

/// Prompt manager for handling templates
pub struct PromptManager {
    env: Environment<'static>,
    templates: HashMap<String, PromptTemplate>,
}

impl PromptManager {
    /// Create a new prompt manager
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            templates: HashMap::new(),
        }
    }

    /// Load templates from a directory
    pub fn load_templates(&mut self, _template_dir: &Path) -> Result<()> {
        // TODO: Implement template loading from directory
        Ok(())
    }

    /// Add a template
    pub fn add_template(&mut self, template: PromptTemplate) -> Result<()> {
        let name = template.name.clone();
        let content = template.content.clone();
        self.env
            .add_template_owned(name.clone(), content)
            .context("Failed to add template")?;
        self.templates.insert(name, template);
        Ok(())
    }

    /// Render a template with the given context
    pub fn render(
        &self,
        template_name: &str,
        context_data: HashMap<String, String>,
    ) -> Result<String> {
        let tmpl = self
            .env
            .get_template(template_name)
            .context("Template not found")?;

        let ctx = context! { data => context_data };
        tmpl.render(ctx).context("Failed to render template")
    }

    /// List all available templates
    pub fn list_templates(&self) -> Vec<&str> {
        self.templates.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for PromptManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_manager_new() {
        let pm = PromptManager::new();
        assert_eq!(pm.list_templates().len(), 0);
    }

    #[test]
    fn test_add_and_render_template() {
        let mut pm = PromptManager::new();
        let template = PromptTemplate {
            name: "test".to_string(),
            content: "Hello {{ data.name }}!".to_string(),
            variables: vec!["name".to_string()],
        };

        pm.add_template(template).unwrap();

        let mut context = HashMap::new();
        context.insert("name".to_string(), "World".to_string());

        let result = pm.render("test", context).unwrap();
        assert_eq!(result, "Hello World!");
    }
}
