mod commands;
pub mod error;
pub mod model;
pub mod parser;
pub mod resolver;
pub mod view;

use tauri_specta::{collect_commands, Builder};

/// Build the tauri-specta Builder with all commands registered. Shared by the
/// runtime (`run`) and the binding-export test so the two never drift.
fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![
        commands::load_conversation,
        commands::initial_url,
        commands::export_html
    ])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();

    let mut tauri_builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        tauri_builder =
            tauri_builder.plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
                use tauri::Emitter;
                if let Some(url) = args.iter().find(|a| a.starts_with("convo://")) {
                    let _ = app.emit("deep-link", url.clone());
                }
            }));
    }

    tauri_builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            #[cfg(desktop)]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                let _ = app.deep_link().register("convo");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod export_bindings {
    use super::*;

    /// Regenerates `src/lib/bindings.ts` from the Rust types on every `cargo test`.
    /// This is how bindings are generated in this (headless) environment instead of
    /// launching the app.
    #[test]
    fn export_typescript_bindings() {
        specta_builder()
            .export(
                specta_typescript::Typescript::default()
                    // Token counts are u64; render them as plain TS `number`.
                    .bigint(specta_typescript::BigIntExportBehavior::Number),
                "../src/lib/bindings.ts",
            )
            .expect("failed to export typescript bindings");
    }
}
