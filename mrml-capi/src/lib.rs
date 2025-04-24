use mrml::mjml::Mjml;
use mrml::prelude::parser::loader::IncludeLoader;
use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
use mrml::prelude::parser::noop_loader::NoopIncludeLoader;
use mrml::prelude::print::Printable;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

fn _mrml_set_last_error<E>(err: Option<E>)
where
    E: ToString,
{
    LAST_ERROR.set(err.map(|err| CString::new(err.to_string()).unwrap()))
}

#[no_mangle]
pub unsafe extern "C" fn mrml_last_error() -> *const c_char {
    LAST_ERROR.with_borrow(|err| {
        if let Some(err) = err {
            err.as_ptr()
        } else {
            std::ptr::null()
        }
    })
}

#[repr(C)]
pub enum ParseResults {
    ParserSuccess,
    ParserInvalidArgument,
    ParserError,
}

#[repr(C)]
pub enum RenderResults {
    RenderSuccess,
    RenderInvalidArgument,
    RenderError,
}

pub struct NoopIncludeLoaderOptions;

#[derive(Clone, Debug, Default)]
pub struct MemoryIncludeLoaderOptions(Box<HashMap<String, String>>);

pub enum ParserIncludeLoaderOptions {
    Noop(NoopIncludeLoaderOptions),
    Memory(MemoryIncludeLoaderOptions),
}

impl Default for ParserIncludeLoaderOptions {
    fn default() -> Self {
        Self::Noop(NoopIncludeLoaderOptions)
    }
}

impl ParserIncludeLoaderOptions {
    fn build(self) -> Box<dyn IncludeLoader + Send + Sync> {
        match self {
            Self::Noop(_) => Box::<NoopIncludeLoader>::default(),
            Self::Memory(MemoryIncludeLoaderOptions(inner)) => {
                Box::new(MemoryIncludeLoader::from(*inner))
            }
        }
    }
}

#[derive(Default)]
pub struct ParserOptions {
    include_loader: ParserIncludeLoaderOptions,
}

pub struct ParserOutput {
    inner: mrml::mjml::Mjml,
}

impl ParserOptions {
    pub fn new(include_loader: Option<ParserIncludeLoaderOptions>) -> Self {
        Self {
            include_loader: include_loader.unwrap_or_default(),
        }
    }
}

impl From<&ParserOptions> for mrml::prelude::parser::ParserOptions {
    fn from(value: &ParserOptions) -> Self {
        let include_loader = match &value.include_loader {
            ParserIncludeLoaderOptions::Noop(_) => {
                ParserIncludeLoaderOptions::Noop(NoopIncludeLoaderOptions).build()
            }
            ParserIncludeLoaderOptions::Memory(options) => {
                ParserIncludeLoaderOptions::Memory(options.clone()).build()
            }
        };

        mrml::prelude::parser::ParserOptions { include_loader }
    }
}

unsafe fn reparse_parsed_output(
    parser_options: *const ParserOptions,
    root: mrml::mjml::Mjml,
) -> Result<mrml::prelude::parser::ParseOutput<Mjml>, mrml::prelude::parser::Error> {
    let input = root.print_dense().unwrap();

    let parser_options_ref = &*parser_options;
    let mrml_parser_options: mrml::prelude::parser::ParserOptions = parser_options_ref.into();

    return mrml::parse_with_options(input, &mrml_parser_options);
}

#[no_mangle]
pub unsafe extern "C" fn new_parser_options(parser: *mut *mut ParserOptions) -> ParseResults {
    let options = ParserIncludeLoaderOptions::default();
    let p = ParserOptions::new(Some(options));
    let boxed = Box::into_raw(Box::new(p));

    *parser = boxed;

    return ParseResults::ParserSuccess;
}

#[no_mangle]
pub unsafe extern "C" fn add_memory_loader(
    parser_options: &mut ParserOptions,
    key: *const c_char,
    value: *const c_char,
) -> ParseResults {
    let key = if let Ok(key) = CStr::from_ptr(key).to_str() {
        key
    } else {
        return ParseResults::ParserInvalidArgument;
    };

    let value = if let Ok(value) = CStr::from_ptr(value).to_str() {
        value
    } else {
        return ParseResults::ParserInvalidArgument;
    };

    match &parser_options.include_loader {
        ParserIncludeLoaderOptions::Noop(_) => {
            let mut map = MemoryIncludeLoaderOptions::default();
            map.0.insert(key.to_string(), value.to_string());
            parser_options.include_loader = ParserIncludeLoaderOptions::Memory(map);
        }
        ParserIncludeLoaderOptions::Memory(map) => {
            let mut new_map = MemoryIncludeLoaderOptions::default();
            map.clone_into(&mut new_map);
            new_map.0.insert(key.to_string(), value.to_string());
            parser_options.include_loader = ParserIncludeLoaderOptions::Memory(new_map);
        }
    }

    return ParseResults::ParserSuccess;
}

#[no_mangle]
pub unsafe extern "C" fn parse_json(
    parser_options: &mut ParserOptions,
    input: *const c_char,
    output: *mut *mut ParserOutput,
) -> ParseResults {
    let input = if let Ok(input) = CStr::from_ptr(input).to_str() {
        input
    } else {
        return ParseResults::ParserInvalidArgument;
    };

    let results = serde_json::from_str::<mrml::mjml::Mjml>(input);

    match results {
        Ok(element) => match reparse_parsed_output(parser_options, element) {
            Ok(mjml) => {
                *output = Box::into_raw(Box::new(ParserOutput {
                    inner: mjml.element,
                }));
                return ParseResults::ParserSuccess;
            }
            Err(err) => {
                _mrml_set_last_error(Some(err));
                return ParseResults::ParserError;
            }
        },
        Err(err) => {
            _mrml_set_last_error(Some(err));
            return ParseResults::ParserError;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn parse_mjml(
    parser_options: &mut ParserOptions,
    input: *const c_char,
    output: *mut *mut ParserOutput,
) -> ParseResults {
    let input = if let Ok(input) = CStr::from_ptr(input).to_str() {
        input
    } else {
        return ParseResults::ParserInvalidArgument;
    };

    let parser_options: mrml::prelude::parser::ParserOptions = (&*parser_options).into();

    match mrml::parse_with_options(input, &parser_options) {
        Ok(mjml) => {
            *output = Box::into_raw(Box::new(ParserOutput {
                inner: mjml.element,
            }));

            return ParseResults::ParserSuccess;
        }
        Err(err) => {
            _mrml_set_last_error(Some(err));
            return ParseResults::ParserError;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn to_html(
    parser_output: *const ParserOutput,
    output: *mut *mut c_char,
) -> RenderResults {
    let render_options = mrml::prelude::render::RenderOptions::default();

    let root = &*parser_output;

    match root.inner.render(&render_options) {
        Ok(s) => {
            *output = CString::new(s).unwrap().into_raw();
            _mrml_set_last_error::<mrml::prelude::render::Error>(None);
            return RenderResults::RenderSuccess;
        }
        Err(err) => {
            _mrml_set_last_error(Some(err));
            return RenderResults::RenderError;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn to_json(
    parser_output: *const ParserOutput,
    output: *mut *mut c_char,
) -> RenderResults {
    let parser_output = &*parser_output;

    match serde_json::to_string(&parser_output.inner) {
        Ok(s) => {
            *output = CString::new(s).unwrap().into_raw();
            _mrml_set_last_error::<mrml::prelude::render::Error>(None);
            return RenderResults::RenderSuccess;
        }
        Err(err) => {
            _mrml_set_last_error(Some(err));
            return RenderResults::RenderError;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn to_mjml(
    parser_output: *const ParserOutput,
    output: *mut *mut c_char,
) -> RenderResults {
    let parser_output = &*parser_output;

    match parser_output.inner.print_dense() {
        Ok(s) => {
            *output = CString::new(s).unwrap().into_raw();
            _mrml_set_last_error::<mrml::prelude::render::Error>(None);
            return RenderResults::RenderSuccess;
        }
        Err(err) => {
            _mrml_set_last_error(Some(err));
            return RenderResults::RenderSuccess;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_parser_options(parser_options: *mut ParserOptions) {
    if !parser_options.is_null() {
        drop(Box::from(parser_options))
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_parser_output(parser_output: *mut ParserOutput) {
    if !parser_output.is_null() {
        drop(Box::from_raw(parser_output));
    }
}
