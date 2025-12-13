use zed::lsp::CompletionKind;
use zed::settings::LspSettings;
use zed::{CodeLabel, CodeLabelSpan};
use zed_extension_api::serde_json::json;
use zed_extension_api::{
    self as zed, current_platform, serde_json, DebugAdapterBinary, DebugTaskDefinition, Os, Result,
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest, Worktree,
};

struct DartBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
}

struct DartExtension;

impl DartExtension {
    fn language_server_binary(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<DartBinary> {
        let binary_settings = LspSettings::for_worktree("dart", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(DartBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = worktree.which("dart") {
            return Ok(DartBinary {
                path,
                args: binary_args,
            });
        }

        Err(
            "dart must be installed from dart.dev/get-dart or pointed to by the LSP binary settings"
                .to_string(),
        )
    }
}

impl zed::Extension for DartExtension {
    fn new() -> Self {
        Self
    }

    /// ref:
    /// https://github.com/zed-industries/zed/blob/main/crates/dap_adapters/src/gdb.rs
    fn get_dap_binary(
        &mut self,
        _adapter_name: String,
        config: DebugTaskDefinition,
        _user_provided_debug_adapter_path: Option<String>,
        worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String> {
        let user_config: serde_json::Value = serde_json::from_str(&config.config)
            .map_err(|e| format!("Failed to parse debug config: {e}"))?;

        let program = user_config
            .get("program")
            .and_then(|v| v.as_str())
            .unwrap_or("lib/main.dart");

        let args = user_config
            .get("args")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .unwrap_or_default();

        let use_fvm = user_config
            .get("useFvm")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // Get debug_mode from user config (flutter or dart)
        let debug_mode = user_config
            .get("type")
            .and_then(|v| v.as_str())
            .filter(|s| !s.trim().is_empty()) // Filter out empty strings
            .ok_or_else(|| "type is required and cannot be empty or null".to_string())?;

        let (os, _) = current_platform();
        let tool = if debug_mode == "flutter" {
            match os {
                Os::Windows => "flutter.bat",
                _ => "flutter",
            }
        } else {
            match os {
                Os::Windows => "dart.bat",
                _ => "dart",
            }
        };

        let (command, arguments) = if use_fvm {
            (
                "fvm".to_string(),
                vec![tool.to_string(), "debug_adapter".to_string()],
            )
        } else {
            (tool.to_string(), vec!["debug_adapter".to_string()])
        };

        let device_id = user_config
            .get("device_id")
            .and_then(|v| v.as_str())
            .unwrap_or("chrome");

        let platform = user_config
            .get("platform")
            .and_then(|v| v.as_str())
            .unwrap_or("web");

        let cwd = user_config
            .get("cwd")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| Some(worktree.root_path()));

        let request = user_config
            .get("request")
            .and_then(|v| v.as_str())
            .unwrap_or("launch");

        let vm_service_uri = user_config
            .get("vmServiceUri")
            .and_then(|v| v.as_str());

        let config_json = json!({
            "type": tool,
            "request": request,
            "vmServiceUri": vm_service_uri,
            "program": program,
            "cwd": cwd.clone().unwrap_or_default(),
            "args": args,
            "flutterMode": "debug",
            "deviceId": device_id,
            "platform": platform,
            "stopOnEntry": false
        })
        .to_string();

        let debug_adapter_binary = DebugAdapterBinary {
            command: Some(command),
            arguments,
            envs: vec![], // Add any Dart-specific env vars if needed
            cwd,
            connection: None,
            request_args: StartDebuggingRequestArguments {
                configuration: config_json,
                request: match request {
                    "attach" => StartDebuggingRequestArgumentsRequest::Attach,
                    _ => StartDebuggingRequestArgumentsRequest::Launch,
                },
            }, // request_args: StartDebuggingRequestArguments:,
        };
        Result::Ok(debug_adapter_binary)
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        config: serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest, String> {
        match config.get("request") {
            Some(v) if v == "launch" => Ok(StartDebuggingRequestArgumentsRequest::Launch),
            Some(v) if v == "attach" => Ok(StartDebuggingRequestArgumentsRequest::Attach),
            Some(value) => Err(format!(
                "Unexpected value for `request` key in Dart debug adapter configuration: {value:?}"
            )),
            None => {
                Err("Missing required `request` field in Dart debug adapter configuration".into())
            }
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let dart_binary = self.language_server_binary(language_server_id, worktree)?;

        Ok(zed::Command {
            command: dart_binary.path,
            args: dart_binary.args.unwrap_or_else(|| {
                vec!["language-server".to_string(), "--protocol=lsp".to_string()]
            }),
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("dart", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "dart": settings
        })))
    }

    fn label_for_completion(
        &self,
        _language_server_id: &zed::LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<CodeLabel> {
        let arrow = " → ";

        match completion.kind? {
            CompletionKind::Class => Some(CodeLabel {
                filter_range: (0..completion.label.len()).into(),
                spans: vec![CodeLabelSpan::literal(
                    completion.label,
                    Some("type".into()),
                )],
                code: String::new(),
            }),
            CompletionKind::Function | CompletionKind::Constructor | CompletionKind::Method => {
                let mut parts = completion.detail.as_ref()?.split(arrow);
                let (name, _) = completion.label.split_once('(')?;
                let parameter_list = parts.next()?;
                let return_type = parts.next()?;
                let fn_name = " a";
                let fat_arrow = " => ";
                let call_expr = "();";

                let code =
                    format!("{return_type}{fn_name}{parameter_list}{fat_arrow}{name}{call_expr}");

                let parameter_list_start = return_type.len() + fn_name.len();

                Some(CodeLabel {
                    spans: vec![
                        CodeLabelSpan::code_range(
                            code.len() - call_expr.len() - name.len()..code.len() - call_expr.len(),
                        ),
                        CodeLabelSpan::code_range(
                            parameter_list_start..parameter_list_start + parameter_list.len(),
                        ),
                        CodeLabelSpan::literal(arrow, None),
                        CodeLabelSpan::code_range(0..return_type.len()),
                    ],
                    filter_range: (0..name.len()).into(),
                    code,
                })
            }
            CompletionKind::Property => {
                let class_start = "class A {";
                let get = " get ";
                let property_end = " => a; }";
                let ty = completion.detail?;
                let name = completion.label;

                let code = format!("{class_start}{ty}{get}{name}{property_end}");
                let name_start = class_start.len() + ty.len() + get.len();

                Some(CodeLabel {
                    spans: vec![
                        CodeLabelSpan::code_range(name_start..name_start + name.len()),
                        CodeLabelSpan::literal(arrow, None),
                        CodeLabelSpan::code_range(class_start.len()..class_start.len() + ty.len()),
                    ],
                    filter_range: (0..name.len()).into(),
                    code,
                })
            }
            CompletionKind::Variable => {
                let name = completion.label;

                Some(CodeLabel {
                    filter_range: (0..name.len()).into(),
                    spans: vec![CodeLabelSpan::literal(name, Some("variable".into()))],
                    code: String::new(),
                })
            }
            _ => None,
        }
    }
}

zed::register_extension!(DartExtension);
