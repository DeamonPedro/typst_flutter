import 'package:flutter/services.dart';

import 'rust/api/typst_compiler.dart' as rust;

class TypstFlutter {
  TypstFlutter._();

  /// Compila um template Typst e retorna os bytes do PDF.
  ///
  /// [template]   — conteúdo do .typ
  /// [inputs]     — Map que será serializado em JSON e injetado no template
  /// [fontAssets] — caminhos de assets Flutter com fontes (.ttf/.otf)
  /// [extraFiles] — arquivos extras referenciados no template
  static Future<Uint8List> compile({
    required String template,
    Map<String, String>? inputs,
    List<String> fontAssets = const [],
    List<({String path, Uint8List bytes})> extraFiles = const [],
  }) async {
    final fonts = <Uint8List>[];
    for (final p in fontAssets) {
      final d = await rootBundle.load(p);
      fonts.add(d.buffer.asUint8List());
    }

    final result = await rust.compile(
      template: template,
      inputs: inputs,
      fonts: fonts,
      extraFiles: extraFiles
          .map((f) => rust.TypstFileInput(path: f.path, data: f.bytes))
          .toList(),
    );

    for (final w in result.warnings) {
      // ignore: avoid_print
      print('[typst] $w');
    }
    return result.pdfBytes;
  }

  static Future<Uint8List> compileAsset({
    required String assetPath,
    Map<String, String>? inputs,
    List<String> fontAssets = const [],
    List<({String path, Uint8List bytes})> extraFiles = const [],
  }) async {
    final source = await rootBundle.loadString(assetPath);
    return compile(
      template: source,
      inputs: inputs,
      fontAssets: fontAssets,
      extraFiles: extraFiles,
    );
  }

  static Future<String> typstVersion() {
    return rust.typstVersion();
  }
}
