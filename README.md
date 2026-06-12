# Zed Dart

A [Dart](https://dart.dev/) extension for [Zed](https://zed.dev).

## Recommended Configuration

To make the most of the Dart LSP in Zed, you can configure it to automatically organize imports and apply fixes on format.

### Settings (`settings.json`)

Add the following to your `settings.json` to enable productivity features like organizing imports on save:

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

### Key Bindings (`keymap.json`)

You can bind specific LSP actions to keyboard shortcuts. For example, to manually trigger "Sort Members":

```json
[
  {
    "context": "Editor && mode == full && language == Dart",
    "bindings": {
      "ctrl-alt-s": ["editor:apply_code_action", "source.sortMembers"]
    }
  }
]
```

## Documentation

See:
- [Zed Dart Language Docs](https://zed.dev/docs/languages/dart)
- [Dart LSP Support Docs](https://github.com/dart-lang/sdk/blob/main/pkg/analysis_server/tool/lsp_spec/README.md)

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.
