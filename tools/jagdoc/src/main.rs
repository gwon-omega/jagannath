//! jagdoc - Jagannath Documentation Generator
//!
//! Generates HTML documentation from Jagannath source code.
//! Extracts doc comments (/// and //!) and generates browsable documentation.
//!
//! Usage:
//!   jagdoc [OPTIONS] [path]

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{error, info};

/// Jagannath Documentation Generator - प्रलेखन निर्माता
#[derive(Parser)]
#[command(name = "jagdoc")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Generate documentation for Jagannath projects")]
struct Cli {
    /// Path to document (default: current directory)
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Output directory
    #[arg(short, long, default_value = "lakṣya/doc")]
    output: PathBuf,

    /// Open documentation in browser after generating
    #[arg(long)]
    open: bool,

    /// Include private items
    #[arg(long)]
    document_private: bool,

    /// Generate in Sanskrit
    #[arg(long)]
    sanskrit: bool,

    /// Theme (light, dark, auto)
    #[arg(long, default_value = "auto")]
    theme: String,
}

fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    info!("जगन्नाथ प्रलेखन निर्माता (Jagannath Documentation Generator)");

    if let Err(e) = generate_documentation(&cli) {
        error!("Documentation generation failed: {}", e);
        std::process::exit(1);
    }

    info!("Documentation generated at: {}", cli.output.display());

    if cli.open {
        let index = cli.output.join("index.html");
        if let Err(e) = open::that(&index) {
            error!("Failed to open browser: {}", e);
        }
    }
}

fn generate_documentation(cli: &Cli) -> Result<(), String> {
    // Create output directory
    std::fs::create_dir_all(&cli.output)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    // Collect all .jag files
    let sources = collect_sources(&cli.path)?;
    info!("Found {} source files", sources.len());

    // Parse and extract documentation
    let mut modules = Vec::new();
    for source_path in &sources {
        let doc = extract_documentation(source_path, cli.document_private)?;
        modules.push(doc);
    }

    // Generate HTML
    generate_html(&modules, &cli.output, cli.sanskrit, &cli.theme)?;

    Ok(())
}

fn collect_sources(path: &PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut sources = Vec::new();

    if path.is_file() {
        sources.push(path.clone());
    } else if path.is_dir() {
        collect_sources_recursive(path, &mut sources)?;
    }

    Ok(sources)
}

fn collect_sources_recursive(dir: &PathBuf, sources: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            collect_sources_recursive(&path, sources)?;
        } else if path.extension().map_or(false, |ext| ext == "jag") {
            sources.push(path);
        }
    }

    Ok(())
}

/// Documentation for a module
#[derive(Debug)]
struct ModuleDoc {
    name: String,
    path: PathBuf,
    module_doc: Option<String>,
    items: Vec<ItemDoc>,
}

/// Documentation for an item (function, struct, etc.)
#[derive(Debug)]
struct ItemDoc {
    kind: ItemKind,
    name: String,
    doc: Option<String>,
    signature: String,
    visibility: Visibility,
    examples: Vec<String>,
    params: Vec<ParamDoc>,
    returns: Option<String>,
}

#[derive(Debug, Clone, Copy)]
enum ItemKind {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Constant,
    Type,
}

#[derive(Debug, Clone, Copy)]
enum Visibility {
    Public,
    Private,
}

#[derive(Debug)]
struct ParamDoc {
    name: String,
    ty: String,
    karaka: Option<String>,
    doc: Option<String>,
}

fn extract_documentation(path: &PathBuf, include_private: bool) -> Result<ModuleDoc, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let name = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // TODO: Use actual parser to extract documentation
    // For now, simple regex-based extraction

    let mut module_doc = None;
    let mut items = Vec::new();

    let mut current_doc = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Module doc (//!)
        if trimmed.starts_with("//!") {
            let doc_line = trimmed.strip_prefix("//!").unwrap_or("").trim();
            if module_doc.is_none() {
                module_doc = Some(String::new());
            }
            if let Some(ref mut doc) = module_doc {
                if !doc.is_empty() {
                    doc.push('\n');
                }
                doc.push_str(doc_line);
            }
            continue;
        }

        // Item doc (///)
        if trimmed.starts_with("///") {
            let doc_line = trimmed.strip_prefix("///").unwrap_or("").trim();
            current_doc.push(doc_line.to_string());
            continue;
        }

        // Check for item declarations
        if trimmed.starts_with("pub ")
            || trimmed.starts_with("kāryakrama ")
            || trimmed.starts_with("prakāra ")
            || trimmed.starts_with("gaṇa ")
            || trimmed.starts_with("guṇa ")
        {
            let is_public = trimmed.starts_with("pub ");
            if !is_public && !include_private {
                current_doc.clear();
                continue;
            }

            let (kind, rest) = if trimmed.contains("kāryakrama ") {
                (ItemKind::Function, extract_after(trimmed, "kāryakrama "))
            } else if trimmed.contains("prakāra ") {
                (ItemKind::Struct, extract_after(trimmed, "prakāra "))
            } else if trimmed.contains("gaṇa ") {
                (ItemKind::Enum, extract_after(trimmed, "gaṇa "))
            } else if trimmed.contains("guṇa ") {
                (ItemKind::Trait, extract_after(trimmed, "guṇa "))
            } else {
                current_doc.clear();
                continue;
            };

            let item_name = rest
                .split(|c: char| {
                    !c.is_alphanumeric()
                        && c != '_'
                        && c != 'ā'
                        && c != 'ī'
                        && c != 'ū'
                        && c != 'ṛ'
                        && c != 'ṅ'
                        && c != 'ñ'
                        && c != 'ṭ'
                        && c != 'ḍ'
                        && c != 'ṇ'
                        && c != 'ś'
                        && c != 'ṣ'
                })
                .next()
                .unwrap_or("unknown")
                .to_string();

            let doc = if current_doc.is_empty() {
                None
            } else {
                Some(current_doc.join("\n"))
            };

            items.push(ItemDoc {
                kind,
                name: item_name,
                doc,
                signature: trimmed.to_string(),
                visibility: if is_public {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
                examples: Vec::new(),
                params: Vec::new(),
                returns: None,
            });

            current_doc.clear();
        } else {
            // Non-doc, non-item line - clear accumulated doc
            if !trimmed.is_empty() && !trimmed.starts_with("//") {
                current_doc.clear();
            }
        }
    }

    Ok(ModuleDoc {
        name,
        path: path.clone(),
        module_doc,
        items,
    })
}

fn extract_after<'a>(s: &'a str, pattern: &str) -> &'a str {
    s.find(pattern)
        .map(|idx| &s[idx + pattern.len()..])
        .unwrap_or("")
}

fn generate_html(
    modules: &[ModuleDoc],
    output: &PathBuf,
    sanskrit: bool,
    theme: &str,
) -> Result<(), String> {
    // Generate index.html
    let index_html = generate_index_html(modules, sanskrit, theme);
    std::fs::write(output.join("index.html"), index_html)
        .map_err(|e| format!("Failed to write index.html: {}", e))?;

    // Generate module pages
    for module in modules {
        let module_html = generate_module_html(module, sanskrit, theme);
        let filename = format!("{}.html", module.name);
        std::fs::write(output.join(&filename), module_html)
            .map_err(|e| format!("Failed to write {}: {}", filename, e))?;
    }

    // Generate CSS
    let css = generate_css(theme);
    std::fs::write(output.join("style.css"), css)
        .map_err(|e| format!("Failed to write style.css: {}", e))?;

    Ok(())
}

fn generate_index_html(modules: &[ModuleDoc], sanskrit: bool, _theme: &str) -> String {
    let title = if sanskrit {
        "प्रलेखन"
    } else {
        "Documentation"
    };
    let modules_title = if sanskrit {
        "विभागाः"
    } else {
        "Modules"
    };

    let module_list: String = modules
        .iter()
        .map(|m| format!(r#"<li><a href="{}.html">{}</a></li>"#, m.name, m.name))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"<!DOCTYPE html>
<html lang="sa">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - Jagannath</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <header>
        <h1>जगन्नाथ {title}</h1>
        <nav>
            <a href="index.html">गृहम् (Home)</a>
        </nav>
    </header>
    <main>
        <section class="modules">
            <h2>{modules_title}</h2>
            <ul>
                {module_list}
            </ul>
        </section>
    </main>
    <footer>
        <p>Generated by jagdoc - जगन्नाथ प्रलेखन निर्माता</p>
    </footer>
</body>
</html>"#
    )
}

fn generate_module_html(module: &ModuleDoc, sanskrit: bool, _theme: &str) -> String {
    let functions_title = if sanskrit {
        "कार्यक्रमाः"
    } else {
        "Functions"
    };
    let structs_title = if sanskrit {
        "प्रकाराः"
    } else {
        "Structs"
    };
    let enums_title = if sanskrit { "गणाः" } else { "Enums" };

    let mut functions = String::new();
    let mut structs = String::new();
    let mut enums = String::new();

    for item in &module.items {
        let doc = item.doc.as_deref().unwrap_or("");
        let html = format!(
            r#"
        <article class="item">
            <h3><code>{}</code></h3>
            <pre class="signature">{}</pre>
            <div class="doc">{}</div>
        </article>
        "#,
            item.name, item.signature, doc
        );

        match item.kind {
            ItemKind::Function => functions.push_str(&html),
            ItemKind::Struct => structs.push_str(&html),
            ItemKind::Enum => enums.push_str(&html),
            _ => {}
        }
    }

    let module_doc = module.module_doc.as_deref().unwrap_or("");

    format!(
        r#"<!DOCTYPE html>
<html lang="sa">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - Jagannath</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <header>
        <h1>{}</h1>
        <nav>
            <a href="index.html">← गृहम् (Home)</a>
        </nav>
    </header>
    <main>
        <section class="module-doc">
            <p>{}</p>
        </section>

        <section class="functions">
            <h2>{}</h2>
            {}
        </section>

        <section class="structs">
            <h2>{}</h2>
            {}
        </section>

        <section class="enums">
            <h2>{}</h2>
            {}
        </section>
    </main>
    <footer>
        <p>Generated by jagdoc</p>
    </footer>
</body>
</html>"#,
        module.name,
        module.name,
        module_doc,
        functions_title,
        functions,
        structs_title,
        structs,
        enums_title,
        enums
    )
}

fn generate_css(theme: &str) -> String {
    let (bg, fg, code_bg) = match theme {
        "dark" => ("#1a1a2e", "#eaeaea", "#16213e"),
        "light" => ("#ffffff", "#1a1a2e", "#f5f5f5"),
        _ => ("#ffffff", "#1a1a2e", "#f5f5f5"), // auto defaults to light
    };

    format!(
        r#"
:root {{
    --bg-color: {bg};
    --fg-color: {fg};
    --code-bg: {code_bg};
    --accent: #ff6b6b;
    --accent-2: #4ecdc4;
}}

* {{
    box-sizing: border-box;
}}

body {{
    font-family: 'Noto Sans', 'Noto Sans Devanagari', sans-serif;
    background: var(--bg-color);
    color: var(--fg-color);
    margin: 0;
    padding: 0;
    line-height: 1.6;
}}

header {{
    background: linear-gradient(135deg, var(--accent), var(--accent-2));
    padding: 2rem;
    color: white;
}}

header h1 {{
    margin: 0;
}}

nav a {{
    color: white;
    text-decoration: none;
    margin-right: 1rem;
}}

main {{
    max-width: 900px;
    margin: 2rem auto;
    padding: 0 1rem;
}}

.item {{
    background: var(--code-bg);
    padding: 1rem;
    margin: 1rem 0;
    border-radius: 8px;
    border-left: 4px solid var(--accent);
}}

.signature {{
    background: var(--bg-color);
    padding: 0.5rem;
    overflow-x: auto;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
}}

footer {{
    text-align: center;
    padding: 2rem;
    opacity: 0.7;
}}

h2 {{
    border-bottom: 2px solid var(--accent);
    padding-bottom: 0.5rem;
}}

code {{
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    background: var(--code-bg);
    padding: 0.2rem 0.4rem;
    border-radius: 4px;
}}
"#
    )
}
