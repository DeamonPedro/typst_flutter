# typst_flutter

A Flutter FFI plugin for compiling [Typst](https://typst.app/) templates to PDF.

## Features

- Compile Typst templates to PDF
- Pass dynamic data via `sys.inputs`
- Load fonts from Flutter assets
- Load template files from assets
- Works on Android, iOS, Linux, macOS, Windows, and Web

## Usage

```dart
import 'package:typst_flutter/typst_flutter.dart';

void main() async {
  final pdf = await TypstFlutter.compile(
    template: '''
= Hello, #sys.inputs.at("name", default: "World")!

This is a Typst document compiled in Flutter.
''',
    inputs: {'name': 'Flutter'},
  );
  
  // pdf is Uint8List with PDF bytes
}
```

### Loading from Assets

```dart
final pdf = await TypstFlutter.compileAsset(
  assetPath: 'assets/templates/my_template.typ',
  inputs: {'title': 'My Report'},
);
```

### Adding Custom Fonts

```dart
final pdf = await TypstFlutter.compile(
  template: '''
#set text(font: "Roboto")
= Hello World!
''',
  fontAssets: ['assets/fonts/Roboto-Regular.ttf'],
);
```

## API

### `TypstFlutter.compile()`

| Parameter | Type | Description |
|-----------|------|-------------|
| `template` | `String` | Typst template content |
| `inputs` | `Map<String, String>?` | Data injected as `sys.inputs` |
| `fontAssets` | `List<String>` | Asset paths for fonts |
| `extraFiles` | `List<(path, bytes)>` | Extra files (images, sub-templates) |

Returns `Uint8List` with PDF bytes.

### `TypstFlutter.compileAsset()`

Same as `compile()` but loads the template from a Flutter asset.

### `TypstFlutter.typstVersion()`

Returns the Typst version as string.

## Example

See the `example/` directory for a complete demo app with:
- Live Typst editor
- PDF preview using pdfrx
- Asset loading demonstration
- Integration tests

## Requirements

- Flutter 3.x
- Rust toolchain (for building the plugin)

## Building

```bash
# Build debug
make build

# Run example
make run-example-linux
```

## License

MIT
