use lazy_static::lazy_static;
use parity_wasm::elements::{ExportEntry, Internal, Module as WasmModule, Type};
use swc_common::DUMMY_SP;
use swc_ecmascript::ast::{
    Decl, ExportDecl, FnDecl, Function, Ident, Module, ModuleDecl, ModuleItem, Param, Pat,
    TsEntityName, TsKeywordType, TsKeywordTypeKind, TsQualifiedName, TsType, TsTypeAnn, TsTypeRef,
    VarDecl, VarDeclKind, VarDeclarator,
};
use swc_ecmascript::codegen::text_writer::JsWriter;
use swc_ecmascript::codegen::Emitter;

lazy_static! {
    static ref NUMBER_TYPE: TsTypeAnn = TsTypeAnn {
        span: DUMMY_SP,
        type_ann: Box::new(TsType::TsKeywordType(TsKeywordType {
            kind: TsKeywordTypeKind::TsNumberKeyword,
            span: DUMMY_SP,
        })),
    };
}

/// - `name`: The name of the variable.
/// - `type_member`: The member of the WebAssembly namespace for the variable's type, e.g. 'Memory' for WebAssembly.Memory.
fn wasm_var_decl(name: &str, type_member: &str) -> VarDecl {
    VarDecl {
        declare: false,
        kind: VarDeclKind::Var,
        span: DUMMY_SP,
        decls: vec![VarDeclarator {
            name: Pat::Ident(Ident {
                sym: name.into(),
                span: DUMMY_SP,
                optional: false,
                type_ann: Some(TsTypeAnn {
                    span: DUMMY_SP,
                    type_ann: Box::new(TsType::TsTypeRef(TsTypeRef {
                        span: DUMMY_SP,
                        type_name: TsEntityName::TsQualifiedName(Box::new(TsQualifiedName {
                            left: TsEntityName::Ident(Ident::new("WebAssembly".into(), DUMMY_SP)),
                            right: Ident::new(type_member.into(), DUMMY_SP),
                        })),
                        type_params: None,
                    })),
                }),
            }),
            init: None,
            definite: false,
            span: DUMMY_SP,
        }],
    }
}

fn get_decl_for_export(module: &WasmModule, export: &ExportEntry) -> Decl {
    match export.internal() {
        Internal::Function(index) => {
            let func = &module
                .function_section()
                .expect("expected function section")
                .entries()[*index as usize];
            let Type::Function(fn_type) = &module
                .type_section()
                .expect("expected type section")
                .types()[func.type_ref() as usize];

            Decl::Fn(FnDecl {
                declare: false,
                function: Function {
                    body: None,
                    span: DUMMY_SP,
                    decorators: Vec::new(),
                    is_async: false,
                    is_generator: false,
                    params: fn_type
                        .params()
                        .iter()
                        .enumerate()
                        .map(|(i, _)| Param {
                            span: DUMMY_SP,
                            decorators: Vec::new(),
                            pat: Pat::Ident(Ident {
                                optional: false,
                                span: DUMMY_SP,
                                sym: format!("arg{}", i).into(),
                                type_ann: Some(NUMBER_TYPE.clone()),
                            }),
                        })
                        .collect(),
                    return_type: Some(NUMBER_TYPE.clone()),
                    type_params: None,
                },
                ident: Ident::new(export.field().into(), DUMMY_SP),
            })
        }
        Internal::Memory(_) => Decl::Var(wasm_var_decl(export.field(), "Memory")),
        Internal::Table(_) => Decl::Var(wasm_var_decl(export.field(), "Table")),
        Internal::Global(_) => Decl::Var(wasm_var_decl(export.field(), "Global")),
    }
}

pub fn convert_to_ast(wasm: &[u8]) -> anyhow::Result<Module> {
    let wasm_module: WasmModule = parity_wasm::deserialize_buffer(wasm)?;

    Ok(Module {
        shebang: None,
        span: DUMMY_SP,
        body: wasm_module
            .export_section()
            .map(|section| {
                section
                    .entries()
                    .iter()
                    .map(|export| {
                        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
                            span: DUMMY_SP,
                            decl: get_decl_for_export(&wasm_module, export),
                        }))
                    })
                    .collect()
            })
            .unwrap_or(Vec::new()),
    })
}

pub fn convert(wasm: &[u8]) -> anyhow::Result<String> {
    let js_module = convert_to_ast(wasm)?;

    let mut output = Vec::new();

    {
        let mut emitter = Emitter {
            cfg: Default::default(),
            cm: Default::default(),
            comments: None,
            wr: Box::new(JsWriter::new(
                Default::default(),
                "\n",
                &mut output,
                Default::default(),
            )),
        };

        emitter.emit_module(&js_module).unwrap();
    }

    Ok(String::from_utf8(output)?)
}
