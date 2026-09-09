# Zed Dart

A [Dart](https://dart.dev/) and [Flutter](https://flutter.dev/) extension for [Zed](https://zed.dev).

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

## Debugging

This extension provides DAP (Debug Adapter Protocol) support for both Flutter and Dart applications via `.zed/debug.json`.

### Flutter Debugging

To run and debug a Flutter application:

```json
[
  {
    "adapter": "flutter",
    "label": "Launch Flutter App",
    "program": "lib/main.dart",
    "deviceId": "macos"
  }
]
```

### Attaching to an Existing Session

To attach to a running Flutter application:

```json
[
  {
    "adapter": "flutter",
    "label": "Attach Flutter App",
    "request": "attach",
    "vmServiceUri": "http://127.0.0.1:12345/ws"
  }
]
```

### Dart CLI Application

To debug a standalone Dart application:

```json
[
  {
    "adapter": "Dart",
    "label": "Launch Dart App",
    "program": "bin/main.dart"
  }
]
```

### Configuration Options

| Option | Description |
| --- | --- |
| `adapter` | `"flutter"`, `"Flutter"`, or `"Dart"`. |
| `program` | Entrypoint Dart file (defaults to `lib/main.dart`). |
| `deviceId` | Target device (e.g. `"chrome"`, `"macos"`, simulator ID). |
| `flutterMode` | Mode: `"debug"`, `"profile"`, `"release"`, or `"test"`. |
| `useFvm` | Whether to use [FVM](https://fvm.app/) (`true`/`false`). |
| `flutterPath` | Custom path to the `flutter` executable. |
| `args` | Arguments passed to your application. |
| `toolArgs` | Arguments passed directly to `flutter` CLI (e.g. `--flavor`). |
| `cwd` | Working directory for the debug session. |

## Documentation

See:
- [Zed Dart Language Docs](https://zed.dev/docs/languages/dart)
- [Dart LSP Support Docs](https://github.com/dart-lang/sdk/blob/main/pkg/analysis_server/tool/lsp_spec/README.md)

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.
