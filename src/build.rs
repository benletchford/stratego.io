use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use base64::Engine;

/// Check whether the client dist directory has already been built.
pub fn needs_build(client_dir: &Path) -> bool {
    !client_dir.join("dist").join("index.html").exists()
}

/// Build the client WASM frontend: generate graphics CSS, compile to wasm, run wasm-bindgen,
/// and assemble the dist directory.
pub fn build_client(project_root: &Path, release: bool) -> Result<(), String> {
    let client_dir = project_root.join("client");
    let graphics_dir = client_dir.join("graphics");
    let css_output = client_dir.join("style").join("graphics.css");
    let dist_dir = client_dir.join("dist");

    // 1. Generate graphics CSS
    tracing::info!("Generating graphics.css from SVGs...");
    generate_graphics_css(&graphics_dir, &css_output)?;

    // 2. Compile client to wasm
    tracing::info!("Compiling client to wasm32...");
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--target", "wasm32-unknown-unknown", "-p", "client"]);
    if release {
        cmd.arg("--release");
    }
    cmd.current_dir(project_root);

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to run cargo build: {}", e))?;
    if !status.success() {
        return Err(format!("cargo build (wasm) failed: {}", status));
    }

    // 3. Run wasm-bindgen
    let profile = if release { "release" } else { "debug" };
    let wasm_file = project_root
        .join("target")
        .join("wasm32-unknown-unknown")
        .join(profile)
        .join("client.wasm");

    let wasm_bindgen = ensure_wasm_bindgen_cli(project_root)?;

    tracing::info!("Running wasm-bindgen...");
    let status = Command::new(&wasm_bindgen)
        .args([
            "--target", "web",
            "--out-dir",
        ])
        .arg(&dist_dir)
        .arg("--no-typescript")
        .arg(&wasm_file)
        .status()
        .map_err(|e| format!("Failed to run wasm-bindgen: {}", e))?;
    if !status.success() {
        return Err(format!("wasm-bindgen failed: {}", status));
    }

    // 4. Assemble dist: copy index.html and style/
    let dist_style = dist_dir.join("style");
    fs::create_dir_all(&dist_style)
        .map_err(|e| format!("Failed to create dist/style: {}", e))?;

    // Generate index.html that loads the wasm
    let index_html = generate_index_html(&dist_dir)?;
    fs::write(dist_dir.join("index.html"), index_html)
        .map_err(|e| format!("Failed to write index.html: {}", e))?;

    // Copy CSS files
    copy_dir_contents(&client_dir.join("style"), &dist_style)?;

    // Copy favicon if it exists
    let favicon = client_dir.join("favicon.ico");
    if favicon.exists() {
        fs::copy(&favicon, dist_dir.join("favicon.ico"))
            .map_err(|e| format!("Failed to copy favicon: {}", e))?;
    }

    tracing::info!("Client build complete.");
    Ok(())
}

/// Find the wasm-bindgen JS and wasm filenames in the dist directory.
fn generate_index_html(dist_dir: &Path) -> Result<String, String> {
    let entries = fs::read_dir(dist_dir)
        .map_err(|e| format!("Failed to read dist dir: {}", e))?;

    let mut js_file = None;
    let mut wasm_file = None;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(".js") {
            js_file = Some(name);
        } else if name.ends_with("_bg.wasm") {
            wasm_file = Some(name);
        }
    }

    let js = js_file.ok_or("No .js file found in dist")?;
    let wasm = wasm_file.ok_or("No _bg.wasm file found in dist")?;

    Ok(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <title>stratego.io</title>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="description" content="Open source HTML5 app that is a recreation of the classic game of Stratego">
  <link rel="icon" type="image/x-icon" href="/favicon.ico" />
  <link href="https://fonts.googleapis.com/css?family=Open+Sans" rel="stylesheet" type="text/css">
  <link rel="stylesheet" type="text/css" href="/style/graphics.css">
  <link rel="stylesheet" type="text/css" href="/style/main.css">
  <link rel="modulepreload" href="/{js}">
  <link rel="preload" href="/{wasm}" as="fetch" type="application/wasm" crossorigin>
</head>
<body class="pace-done">
<script type="module">
import init from '/{js}';
await init({{ module_or_path: '/{wasm}' }});
</script>
</body>
</html>"#,
        js = js,
        wasm = wasm,
    ))
}

fn copy_dir_contents(src: &Path, dst: &Path) -> Result<(), String> {
    let entries = fs::read_dir(src)
        .map_err(|e| format!("Failed to read {}: {}", src.display(), e))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let file_type = entry.file_type().map_err(|e| format!("Failed to get file type: {}", e))?;
        let dest_path = dst.join(entry.file_name());
        if file_type.is_file() {
            fs::copy(entry.path(), &dest_path)
                .map_err(|e| format!("Failed to copy {}: {}", entry.path().display(), e))?;
        }
    }
    Ok(())
}

/// Ensure wasm-bindgen-cli is available with the correct version, installing if needed.
fn ensure_wasm_bindgen_cli(project_root: &Path) -> Result<String, String> {
    let required_version = get_wasm_bindgen_version(project_root)?;

    // Check if already installed with correct version
    if let Ok(output) = Command::new("wasm-bindgen").arg("--version").output() {
        if output.status.success() {
            let version_output = String::from_utf8_lossy(&output.stdout);
            // Output is like "wasm-bindgen 0.2.114"
            if version_output.contains(&required_version) {
                return Ok("wasm-bindgen".to_string());
            }
            tracing::info!(
                "wasm-bindgen version mismatch (need {}), reinstalling...",
                required_version
            );
        }
    } else {
        tracing::info!("wasm-bindgen-cli not found, installing...");
    }

    let status = Command::new("cargo")
        .args(["install", "wasm-bindgen-cli", "--version", &required_version])
        .status()
        .map_err(|e| format!("Failed to install wasm-bindgen-cli: {}", e))?;

    if !status.success() {
        return Err(format!(
            "Failed to install wasm-bindgen-cli {}. Install manually: cargo install wasm-bindgen-cli --version {}",
            required_version, required_version
        ));
    }

    Ok("wasm-bindgen".to_string())
}

/// Read the wasm-bindgen version from Cargo.lock.
fn get_wasm_bindgen_version(project_root: &Path) -> Result<String, String> {
    let lock_path = project_root.join("Cargo.lock");
    let content = fs::read_to_string(&lock_path)
        .map_err(|e| format!("Failed to read Cargo.lock: {}", e))?;

    // Parse: find [[package]] with name = "wasm-bindgen" and extract version
    let mut in_wasm_bindgen = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[[package]]" {
            in_wasm_bindgen = false;
        } else if trimmed == r#"name = "wasm-bindgen""# {
            in_wasm_bindgen = true;
        } else if in_wasm_bindgen && trimmed.starts_with("version = ") {
            let version = trimmed
                .strip_prefix("version = \"")
                .and_then(|s| s.strip_suffix('"'))
                .ok_or("Failed to parse wasm-bindgen version from Cargo.lock")?;
            return Ok(version.to_string());
        }
    }

    Err("wasm-bindgen not found in Cargo.lock".to_string())
}

/// Generate graphics.css from SVG source files.
pub fn generate_graphics_css(graphics_dir: &Path, output_path: &Path) -> Result<(), String> {
    let mut rules: BTreeMap<String, String> = BTreeMap::new();
    let colors = color_map();

    // 1. Top-level SVGs (board, board-no-trees, grass1, grass2, tree1-3)
    let top_level_files = [
        "board.svg",
        "board-no-trees.svg",
        "grass1.svg",
        "grass2.svg",
        "tree1.svg",
        "tree2.svg",
        "tree3.svg",
    ];

    for filename in &top_level_files {
        let path = graphics_dir.join(filename);
        let svg = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
        let class_name = filename.trim_end_matches(".svg");
        let data_uri = svg_to_data_uri(&svg);
        let rule = format!(
            ".image-{} {{ background-image: url(\"{}\"); background-repeat: no-repeat; }}",
            class_name, data_uri
        );
        rules.insert(format!("0-{}", class_name), rule);
    }

    // 2. Piece SVGs (pieces/*.colors-black-blue-red.svg)
    let pieces_dir = graphics_dir.join("pieces");
    let piece_files = collect_color_svg_files(&pieces_dir)?;

    for filename in &piece_files {
        let path = pieces_dir.join(filename);
        let svg = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
        let rank = filename
            .strip_suffix(".colors-black-blue-red.svg")
            .expect("filename should have expected suffix");

        for (color_name, color_value) in &colors {
            let colored_svg = replace_color(&svg, color_value);
            let data_uri = svg_to_data_uri(&colored_svg);
            let class_name = format!("{}-{}", rank, color_name);
            let rule = format!(
                ".image-{} {{ background-image: url(\"{}\"); background-repeat: no-repeat; }}",
                class_name, data_uri
            );
            rules.insert(format!("1-{}-{}", rank, color_name), rule);
        }
    }

    // 3. Rank SVGs (pieces/ranks/*.colors-black-blue-red.svg)
    let ranks_dir = pieces_dir.join("ranks");
    let rank_files = collect_color_svg_files(&ranks_dir)?;

    for filename in &rank_files {
        let path = ranks_dir.join(filename);
        let svg = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
        let rank_id = filename
            .strip_suffix(".colors-black-blue-red.svg")
            .expect("filename should have expected suffix");

        for (color_name, color_value) in &colors {
            let colored_svg = replace_color(&svg, color_value);
            let data_uri = svg_to_data_uri(&colored_svg);
            let class_name = format!("{}-{}", rank_id, color_name);
            let rule = format!(
                ".image-{} {{ background-image: url(\"{}\"); background-repeat: no-repeat; }}",
                class_name, data_uri
            );
            rules.insert(format!("2-{}-{}", rank_id, color_name), rule);
        }
    }

    // Write output
    if let Some(output_dir) = output_path.parent() {
        fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create dir {}: {}", output_dir.display(), e))?;
    }

    let css: String = rules.values().cloned().collect::<Vec<_>>().join("\n");
    fs::write(output_path, &css)
        .map_err(|e| format!("Failed to write {}: {}", output_path.display(), e))?;

    tracing::info!("Wrote {} CSS rules to {}", rules.len(), output_path.display());
    Ok(())
}

fn collect_color_svg_files(dir: &Path) -> Result<Vec<String>, String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read {}: {}", dir.display(), e))?;
    let mut files: Vec<String> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".colors-black-blue-red.svg") && entry.file_type().ok()?.is_file() {
                Some(name)
            } else {
                None
            }
        })
        .collect();
    files.sort();
    Ok(files)
}

fn svg_to_data_uri(svg: &str) -> String {
    let b64 = base64::engine::general_purpose::STANDARD.encode(svg.as_bytes());
    format!("data:image/svg+xml;base64,{}", b64)
}

fn replace_color(svg: &str, target_color: &str) -> String {
    let result = replace_hex_color_ci(svg, "#000000", target_color);
    let result = replace_short_hex_color(&result, "#000", target_color);
    inject_fill_on_svg_root(&result, target_color)
}

fn inject_fill_on_svg_root(svg: &str, color: &str) -> String {
    let lower = svg.to_lowercase();
    if let Some(svg_start) = lower.find("<svg") {
        if let Some(tag_end) = svg[svg_start..].find('>') {
            let tag_content = &lower[svg_start..svg_start + tag_end];
            if !has_fill_attribute(tag_content) {
                let insert_pos = svg_start + 4;
                let mut result = String::with_capacity(svg.len() + 20);
                result.push_str(&svg[..insert_pos]);
                result.push_str(&format!(" fill=\"{}\"", color));
                result.push_str(&svg[insert_pos..]);
                return result;
            }
        }
    }
    svg.to_string()
}

fn has_fill_attribute(tag: &str) -> bool {
    let mut search_from = 0;
    while let Some(pos) = tag[search_from..].find("fill") {
        let abs_pos = search_from + pos;
        let after = abs_pos + 4;
        if after < tag.len() {
            let next_char = tag.as_bytes()[after];
            if next_char == b'=' {
                return true;
            }
            if next_char == b' ' || next_char == b'\t' {
                let rest = tag[after..].trim_start();
                if rest.starts_with('=') {
                    return true;
                }
            }
            search_from = after;
        } else {
            break;
        }
    }
    false
}

fn replace_hex_color_ci(input: &str, from: &str, to: &str) -> String {
    let from_lower = from.to_lowercase();
    let input_lower = input.to_lowercase();
    let mut result = String::with_capacity(input.len());
    let mut i = 0;
    let bytes = input.as_bytes();
    let from_len = from.len();

    while i < bytes.len() {
        if i + from_len <= bytes.len()
            && input_lower[i..i + from_len] == from_lower[..from_len]
        {
            result.push_str(to);
            i += from_len;
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }
    result
}

fn replace_short_hex_color(input: &str, from: &str, to: &str) -> String {
    let from_lower = from.to_lowercase();
    let input_lower = input.to_lowercase();
    let mut result = String::with_capacity(input.len());
    let mut i = 0;
    let bytes = input.as_bytes();
    let from_len = from.len();

    while i < bytes.len() {
        if i + from_len <= bytes.len()
            && input_lower[i..i + from_len] == from_lower[..from_len]
        {
            let next_is_hex = if i + from_len < bytes.len() {
                bytes[i + from_len].is_ascii_hexdigit()
            } else {
                false
            };

            if !next_is_hex {
                result.push_str(to);
                i += from_len;
            } else {
                result.push(bytes[i] as char);
                i += 1;
            }
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }
    result
}

fn color_map() -> Vec<(&'static str, &'static str)> {
    vec![
        ("black", "#000000"),
        ("blue", "#0000ff"),
        ("red", "#bf0000"),
    ]
}
