use std::collections::HashMap;

use anyhow::{anyhow, Result};
use flutter_rust_bridge::frb;
use typst_as_lib::TypstEngine;
use typst_library::foundations::{Dict, Str, Value};
use typst_pdf::PdfOptions;

// ── Types ──────────────────────────────────────────────────────────────────

/// Extra file (image, .typ include) for the compiler.
/// `path` is the virtual path used in the template (e.g. "logo.png")
pub struct TypstFileInput {
    pub path: String,
    pub data: Vec<u8>,
}

/// Result of a successful compilation.
pub struct CompileResult {
    /// Generated PDF bytes — ready to save or display.
    pub pdf_bytes: Vec<u8>,
    /// Non-fatal warnings emitted by the Typst compiler.
    pub warnings: Vec<String>,
}

// ── Main API ───────────────────────────────────────────────────────────────

/// Compiles a Typst template and returns the PDF bytes.
///
/// - `template`   : main .typ file content (UTF-8)
/// - `inputs`     : data injected as sys.inputs (HashMap -> Dict)
/// - `fonts`      : additional font bytes (.ttf/.otf)
/// - `extra_files`: extra files (images, sub-templates)
#[frb]
pub fn compile(
    template: String,
    inputs: Option<HashMap<String, String>>,
    fonts: Vec<Vec<u8>>,
    extra_files: Vec<TypstFileInput>,
) -> Result<CompileResult> {
    let mut builder = TypstEngine::builder().main_file(template.as_str());

    for font in &fonts {
        builder = builder.fonts([font.as_slice()]);
    }

    if !extra_files.is_empty() {
        let pairs: Vec<(&str, &[u8])> = extra_files
            .iter()
            .map(|f| (f.path.as_str(), f.data.as_slice()))
            .collect();
        builder = builder.with_static_file_resolver(pairs);
    }

    let engine = builder.build();

    let result = match inputs {
        Some(inputs) => {
            let mut dict = Dict::new();
            for (key, value) in inputs {
                dict.insert(Str::from(key), Value::Str(Str::from(value)));
            }
            engine.compile_with_input(dict)
        }
        None => engine.compile(),
    };

    let warnings: Vec<String> = result.warnings.iter().map(|w| format!("{w:?}")).collect();

    let doc = result
        .output
        .map_err(|err| anyhow!("Typst compile error: {err:?}"))?;

    let pdf_bytes =
        typst_pdf::pdf(&doc, &PdfOptions::default()).map_err(|e| anyhow!("PDF error: {e:?}"))?;

    Ok(CompileResult {
        pdf_bytes,
        warnings,
    })
}

/// Simplified version — no extra fonts or files.
/// Uses system fonts (typst-kit-fonts feature).
#[frb]
pub fn compile_simple(
    template: String,
    inputs: Option<HashMap<String, String>>,
) -> Result<CompileResult> {
    compile(template, inputs, vec![], vec![])
}

/// Returns the version of this plugin's own crate (set in Cargo.toml).
/// Fix 4: typst_as_lib::TYPST_VERSION does not exist —
/// use env! to get our own crate version at compile time instead.
#[frb]
pub fn typst_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ── Tests ──────────────────────────────────────────────────────────────────
#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_compile_simple_returns_pdf() {
        let result = compile("= Hello".to_string(), None, vec![], vec![]).unwrap();

        assert_eq!(&result.pdf_bytes[..4], b"%PDF");
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_compile_with_dict_input() {
        let template = r#"
#import sys: inputs
= #inputs.at("titulo", default: "")
"#
        .to_string();

        let inputs = HashMap::from([("titulo".to_string(), "Test".to_string())]);

        let result = compile(template, Some(inputs), vec![], vec![]).unwrap();

        assert_eq!(&result.pdf_bytes[..4], b"%PDF");
    }

    #[test]
    fn test_compile_simple_shortcut() {
        let result = compile_simple("= Hello".to_string(), None).unwrap();

        assert_eq!(&result.pdf_bytes[..4], b"%PDF");
    }

    #[test]
    fn test_invalid_template_returns_error() {
        let result = compile("#funcao_inexistente()".to_string(), None, vec![], vec![]);

        assert!(result.is_err());
    }

    #[test]
    fn test_larger_content_produces_larger_pdf() {
        let small = compile("= A".to_string(), None, vec![], vec![]).unwrap();

        let large_template = (0..30)
            .map(|i| format!("== Section {i}\n\n{}\n", "Lorem ipsum ".repeat(20)))
            .collect::<Vec<_>>()
            .join("\n");

        let large = compile(large_template, None, vec![], vec![]).unwrap();

        assert!(large.pdf_bytes.len() > small.pdf_bytes.len());
    }
}
