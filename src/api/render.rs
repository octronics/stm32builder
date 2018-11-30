//! Generate API

use crate::{
    api::Error,
    helpers::handlebars::{LowOrHighRegister, StrEqHelper},
};
use handlebars::Handlebars;
use serde::Serialize;
use std::fs::File;

/// Render the template with data in the output file
pub fn render(
    data: impl Serialize,
    template: &mut File,
    output: &mut File,
    context: &Context,
) -> Result<(), Error> {
    context
        .handlebars
        .render_template_source_to_write(template, &data, output)
        .map_err(|e| Error::Render(e))?;

    Ok(())
}

/// Rendering context
pub struct Context {
    pub(crate) handlebars: Handlebars,
}

impl Context {
    /// Create a rendering context
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();

        handlebars.register_helper("str_eq", Box::new(StrEqHelper));
        handlebars.register_helper("low_or_high", Box::new(LowOrHighRegister));

        Self { handlebars }
    }
}
