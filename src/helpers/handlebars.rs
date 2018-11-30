use handlebars::{
    Context, Handlebars, Helper, HelperDef, JsonValue, Output, RenderContext, RenderError,
    ScopedJson,
};

#[derive(Copy, Clone)]
pub struct LowOrHighRegister;

impl HelperDef for LowOrHighRegister {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut Output,
    ) -> Result<(), RenderError> {
        let register = h
            .param(0)
            .map(|x| x.value())
            .ok_or_else(|| RenderError::new("Error when getting register name value"))
            .and_then(|x| {
                x.as_str()
                    .ok_or_else(|| RenderError::new("Error when converting register name"))
            })?;
        let field = h
            .param(1)
            .map(|x| x.value())
            .ok_or_else(|| RenderError::new("Error when getting field number value"))
            .and_then(|x| {
                x.as_i64()
                    .ok_or_else(|| RenderError::new("Error when converting field number"))
            })?;

        if field < 0 || field > 15 {
            return Err(RenderError::new("Field must be in between 0 and 15"));
        }

        out.write(register).unwrap();
        if field > 8 {
            out.write("H").unwrap();
        } else {
            out.write("L").unwrap();
        }
        Ok(())
    }
}

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
