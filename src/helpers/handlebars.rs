use handlebars::{
    Context, Handlebars, Helper, HelperDef, JsonValue, Output, RenderContext, RenderError,
    ScopedJson,
};

#[derive(Clone, Copy)]
pub struct StrEqHelper;

impl HelperDef for StrEqHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
    ) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        let a = h
            .param(0)
            .map(|x| x.value())
            .ok_or_else(|| {
                RenderError::new(&format!("`str_eq` helper: Couldn't read first parameter"))
            })
            .and_then(|x| {
                x.as_str().ok_or_else(|| {
                    RenderError::new("`str_eq` helper: Couldn't convert parameter first parameter")
                })
            })?;

        let b = h
            .param(1)
            .map(|x| x.value())
            .ok_or_else(|| RenderError::new("`str_eq` helper: Couldn't read second parameter"))
            .and_then(|x| {
                x.as_str().ok_or_else(|| {
                    RenderError::new("`str_eq` helper: Couldn't convert second parameter")
                })
            })?;

        let result = a == b;
        Ok(Some(ScopedJson::Derived(JsonValue::from(result))))
    }
}
