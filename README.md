# Zed Dart

A [Dart](https://dart.dev/) extension for [Zed](https://zed.dev).

## Recommended Configuration

To make the most of the Dart LSP in Zed, you can configure it to automatically organize imports and apply fixes on format.

### Settings (`settings.json`)

Add the following to your `settings.json` to enable features like organizing imports on save:

```json
{
  "languages": {
    "Dart": {
      "format_on_save": "on",
      "code_actions_on_format": {
        "source.organizeImports": true,
        "source.fixAll": true
      }
    }
  }
}
```

### Debugging tests (`.zed/debug.json`)

Set `"testMode": true` in a debug configuration to run the target file as a test
suite. The extension spawns the debug adapter with `--test`, so the program runs
via `flutter test` / `dart test` (headless) instead of `flutter run` / `dart run`.
Breakpoints, stepping, and variable inspection work as usual.

```json
{
  "label": "Debug current test file",
  "adapter": "Dart",
  "type": "flutter",
  "request": "launch",
  "program": "$ZED_FILE",
  "cwd": "$ZED_DIRNAME",
  "testMode": true,
  "useFvm": true
}
```

Zed task variables such as `$ZED_FILE` and `$ZED_DIRNAME` work in `.zed/debug.json`.
Use `"toolArgs"` to narrow the run, e.g. `"toolArgs": ["--plain-name", "<test name>"]`.
For a pure-Dart package, set `"type": "dart"` instead (`dart debug_adapter --test`).

## Documentation

See:
- [Zed Dart Language Docs](https://zed.dev/docs/languages/dart)
- [Dart LSP Support Docs](https://github.com/dart-lang/sdk/blob/main/pkg/analysis_server/tool/lsp_spec/README.md)

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.
