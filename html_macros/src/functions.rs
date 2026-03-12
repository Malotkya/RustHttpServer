use regex::{RegexBuilder, Regex};
use swc::{config::IsModule, Compiler, PrintArgs};
use swc_common::{errors::Handler, source_map::SourceMap, sync::Lrc, Mark, FileName, GLOBALS};
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::Syntax;
use swc_ecma_transforms_typescript::strip;
use std::sync::LazyLock;

static FUNCTION_REGEX:LazyLock<Regex> = LazyLock::new(||{
    RegexBuilder::new(r"^\s?(?P<async>async)?\s?function\s(?P<name>.*?)\s?\((?P<args>.*?)\)\{(?P<body>.*?)\}$")
        .dot_matches_new_line(true).unicode(true).build().unwrap()
});
static ARROW_REGEX:LazyLock<Regex> = LazyLock::new(||{
    RegexBuilder::new(r"^(?:(?:const|let)\s+(?P<name>.*?)\s?=\s?)?(?P<async>async)?\((?P<args>.*?)\)\s?=>\{(?P<body>.*)}$")
        .dot_matches_new_line(true).unicode(true).build().unwrap()
});
static SINGLE_LINE_REGEX:LazyLock<Regex> = LazyLock::new(||{
    RegexBuilder::new(r"^(?:(?:const|let)\s+(?P<name>.*?)\s?=\s?)?(?P<async>async)?\((?P<args>.*?)\)\s?=>(?P<body>.*?)$")
        .dot_matches_new_line(true).unicode(true).build().unwrap()
});

struct FunctionMatch<'a> {
    is_async: bool,
    name: Option<&'a str>,
    args: Vec<&'a str>,
    body: &'a str,
    single_line:bool
}

fn regex_match_function(value:&str) -> FunctionMatch<'_> {
    let mut single_line:bool = false;
    let captures = match FUNCTION_REGEX.captures(value) {
        Some(cap) => cap,
        None => match ARROW_REGEX.captures(value) {
            Some(cap) => cap,
            None => match SINGLE_LINE_REGEX.captures(value) {
                Some(cap) => {
                    single_line = true;
                    cap
                },
                None => panic!("Unable to match function!")
            }
        }
    };

    FunctionMatch {
        is_async: captures.name("async").is_some(),
        name: captures.name("name").map(|s|s.as_str()),
        args: captures.name("args").unwrap().as_str()
            .split(",").map(|s|s.trim()).collect(),
        body: captures.name("body").unwrap().as_str()
            .trim(),
        single_line
    }
}

pub fn parse_javascript(input:String, source_map:Option<String>) -> proc_macro2::TokenStream {
    let FunctionMatch{name, args, body, single_line, is_async} = regex_match_function(&input);

    let name = match name {
        Some(str) => quote::quote!{Some(#str)},
        None => quote::quote!{None}
    };

    let source_map = match source_map {
        Some(str) => quote::quote!{Some(#str)},
        None => quote::quote!{None}
    };

    let mut updated_body:String;
    if single_line {
        updated_body = String::with_capacity(body.len() + 7);
        updated_body.push_str("return ");
        updated_body.push_str(body);
    } else {
        updated_body = body.to_string()
    };

    quote::quote!{
        function::JsFunction{
            is_async: #is_async
            name: #name,
            args: vec![#( #args )*],
            body: #updated_body,
            source_map: #source_map
        };
    }
}

pub fn compile_ts(input:String) -> (String, String) {
    let cm: Lrc<SourceMap> = Default::default();

    let handler = Handler::with_emitter_writer(
        Box::new(std::io::stderr()),
        Some(cm.clone())
    );

    let fm: Lrc<swc_common::SourceFile> = cm.new_source_file(
        FileName::Custom("rust.js".into()).into(),
        input
    );

    let compiler = Compiler::new(cm.clone());

    return GLOBALS.set(&Default::default(), ||{
        let mut program = compiler.parse_js(
            fm,
            &handler,
            EsVersion::Es2024,
            Syntax::Typescript(Default::default()),
            IsModule::Bool(false),
            Some(compiler.comments())
        ).expect("Parsing Typescript Failed!");

        program.mutate(&mut strip(Mark::new(), Mark::new()));

        let ret = compiler
            .print(
                &program, // ast to print
                PrintArgs::default(),
            )
            .expect("Printing Javascript Failed!");

        return (ret.code, ret.map.unwrap_or(String::new()))
    })
}