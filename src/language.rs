#[cfg(feature = "pt")]
pub mod strings {
  pub const STRING_FAIL_DESERIALIZE: &str = "⚡ A desserialização do arquivo falhou!";
  pub const STRING_FAIL_EMPTY_PATTERNS: &str = "⚡ Os padrões estão vazios!";
  pub const STRING_FAIL_NO_PATTERNS: &str = "⚡ Os padrões não foram encontrados!";
  pub const STRING_FAIL_EXPORT_MIDI: &str = "⚡ A exportação de arquivos MIDI falhou!";
  pub const STRING_EXPORTING: &str = "Exportando... ";
  pub const STRING_EXPORTED: &str = "Exportado";
  pub const STRING_EXPORTED_COMPLETE: &str = "Exportação completa!";
  pub const STRING_PLAYBACK_COMPLETE: &str = "Reprodução completa!";
  pub const STRING_TEMPLATE_EXPORT_COMPLETE: &str = "Padrão exportado!";
  pub const STRING_TEMPLATE_EXPORT_FAIL: &str = "Exportação de Padrão falhou";
  pub const STRING_TITLE: &str = "Chord Composer (Compositor de acorde)";
  pub const STRING_ABOUT: &str = "Ferramenta de composição para arranjos de acordes";
  pub const STRING_ABOUT_PLAY: &str = "Tocar um arranjo";
  pub const STRING_HELP_COMPOSITION_FILE: &str = "O arquivo YAML do arranjo para tocar";
  pub const STRING_HELP_METRONOME: &str = "Reprodução com Metrônomo";
  pub const STRING_ABOUT_EXPORT: &str = "Exportar o padrão de composição para .mid";
  pub const STRING_ABOUT_TEMPLATE_EXPORT: &str = "Exportar o modelo de composição";
  pub const STRING_PATH_TEMPLATE_EXPORT: &str = "A pasta para exportar o padrão YAML";
  pub const STRING_ABOUT_CHORDS: &str = "Mostrar lista de acordes apoiados e os seus intervalos";
  pub const STRING_WARNING_ADDITIONAL: &str = "Comandos adicionais necessitados";
  pub const STRING_WARNING_NOT_FOUND: &str = "Não foi encontrado.";
  pub const STRING_HELP: &str = "Para mais ajuda, digite --help";
  pub const STRING_TIME_REVERSE: &str = "⚡ O tempo não pode fluir ao contrário!";
  pub const STRING_BAD_TIME_SIGNATURE: &str = "⚡ Assinatura de hora ruim! O denominador deve ter pelo menos 1. Numeradores 2, 4, 8, 6, 16, 32 são suportados apenas!";
  pub const STRING_UNREACHABLE_EVENT: &str = "⚡ O evento não pode ser alcançado";
  pub const STRING_FAIL_LOAD_SAMPLER: &str =
    "⚡ Um dos instrumentos não pode ser carregado ou criado para reprodução. Eles existem?";
  pub const STRING_HELP_TICKER_BAR: &str = "Imprime a hora atual em cada mudança de barra.";
  pub const STRING_HELP_TICKER_BEAT: &str = "Imprime a hora atual em cada alteração de batida.";
  pub const STRING_HELP_TICKER_INTERVAL: &str =
    "Imprime a hora atual em cada alteração do intervalo de batida.";
}

#[cfg(feature = "zhn")]
pub mod strings {
  pub const STRING_FAIL_DESERIALIZE: &str = "⚡ 无法反序列化文件!";
  pub const STRING_FAIL_EMPTY_PATTERNS: &str = "⚡ 音乐模式为空!";
  pub const STRING_FAIL_NO_PATTERNS: &str = "⚡ 找不到音乐模式!";
  pub const STRING_FAIL_EXPORT_MIDI: &str = "⚡ 无法导出Midi文件!";
  pub const STRING_EXPORTING: &str = "出口... ";
  pub const STRING_EXPORTED: &str = "出口的";
  pub const STRING_EXPORTED_COMPLETE: &str = "出口完成!";
  pub const STRING_PLAYBACK_COMPLETE: &str = "播放完成!";
  pub const STRING_TEMPLATE_EXPORT_COMPLETE: &str = "出口模板!";
  pub const STRING_TEMPLATE_EXPORT_FAIL: &str = "出口失败模板";
  pub const STRING_TITLE: &str = "Chord Composer (和弦作曲家)";
  pub const STRING_ABOUT: &str = "和弦安排的合成工具";
  pub const STRING_ABOUT_PLAY: &str = "播放音乐";
  pub const STRING_HELP_COMPOSITION_FILE: &str = "要播放的音乐的yaml文件";
  pub const STRING_HELP_METRONOME: &str = "节拍器播放";
  pub const STRING_ABOUT_EXPORT: &str = "将合成模式导出到.mid";
  pub const STRING_ABOUT_TEMPLATE_EXPORT: &str = "导出合成模板";
  pub const STRING_PATH_TEMPLATE_EXPORT: &str = "yaml模板的导出路径";
  pub const STRING_ABOUT_CHORDS: &str = "打印支持的和弦列表及其间隔";
  pub const STRING_WARNING_ADDITIONAL: &str = "需要其他命令.";
  pub const STRING_WARNING_NOT_FOUND: &str = "找不到.";
  pub const STRING_HELP: &str = "更多，使用 --help";
  pub const STRING_TIME_REVERSE: &str = "⚡ 时间不能倒流!";
  pub const STRING_BAD_TIME_SIGNATURE: &str =
    "⚡ 时间签名不好！分母必须至少为1. 仅支持数字, 4, 8, 6, 16, 32";
  pub const STRING_UNREACHABLE_EVENT: &str = "⚡ 无法达到事件";
  pub const STRING_FAIL_LOAD_SAMPLER: &str = "⚡ 无法加载或创建其中一种乐器进行播放。它们存在吗？";
  pub const STRING_HELP_TICKER_BAR: &str = "在每个音乐栏更改上打印当前音乐时间.";
  pub const STRING_HELP_TICKER_BEAT: &str = "在每次音乐节拍变化时打印当前音乐时间.";
  pub const STRING_HELP_TICKER_INTERVAL: &str = "在每个音乐节拍间隔更改时打印当前音乐时间.";
}

//TODO: Is there a better approach for default features?
#[cfg(any(feature = "eng", all(not(feature = "zhn"), not(feature = "pt"))))]
pub mod strings {
  // Errors/Warnings
  pub const STRING_FAIL_DESERIALIZE: &str =
    "⚡ Failed to deserialize file! Does it exist or have composition parameters?";
  pub const STRING_FAIL_EMPTY_PATTERNS: &str = "⚡ Patterns are empty!";
  pub const STRING_FAIL_NO_PATTERNS: &str = "⚡ No patterns found!";
  pub const STRING_FAIL_EXPORT_MIDI: &str = "⚡ Failed to export midi files!";
  pub const STRING_TIME_REVERSE: &str = "⚡ Time cannot reverse!";
  pub const STRING_BAD_TIME_SIGNATURE: &str = "⚡ Bad time signature! Denominator must be at least 1.  Numerators 2, 4, 8, 6, 16, 32 are only supported!";
  pub const STRING_UNREACHABLE_EVENT: &str = "⚡ The event cannot be reached";
  pub const STRING_FAIL_LOAD_SAMPLER: &str =
    "⚡ One of the instruments cannot be loaded or created for playback. Do they exist?";
  pub const STRING_TEMPLATE_EXPORT_FAIL: &str = "⚡ Failed to export template!";
  pub const STRING_WARNING_NOT_FOUND: &str = "⚡ Cannot be found.";
  pub const STRING_WARNING_ADDITIONAL: &str = "⚡ Additional commands required.";

  // Status
  pub const STRING_EXPORTING: &str = "Exporting... ";
  pub const STRING_EXPORTED: &str = "Exported";
  pub const STRING_EXPORTED_COMPLETE: &str = "Export complete!";
  pub const STRING_PLAYBACK_COMPLETE: &str = "Playback complete!";
  pub const STRING_TEMPLATE_EXPORT_COMPLETE: &str = "Exported template!";

  // About
  pub const STRING_TITLE: &str = "Chord Composer";
  pub const STRING_ABOUT: &str =
    "A music composition tool for structuring chord progressions and patterns.";
  pub const STRING_ABOUT_PLAY: &str = "Playback patterns in a composition.";
  pub const STRING_HELP_COMPOSITION_FILE: &str = "The YAML composition arrangement file.";
  pub const STRING_HELP_METRONOME: &str = "Play a metronome during playback.";
  pub const STRING_ABOUT_EXPORT: &str = "Export composition patterns to .MID.";
  pub const STRING_ABOUT_TEMPLATE_EXPORT: &str = "Export a composition arrangement YAML template";
  pub const STRING_PATH_TEMPLATE_EXPORT: &str = "Export path of the YAML template";
  pub const STRING_ABOUT_CHORDS: &str = "Print the list of supported chords and their intervals.";
  pub const STRING_HELP_TICKER_BAR: &str = "Prints the current time on each bar change.";
  pub const STRING_HELP_TICKER_BEAT: &str = "Prints the current time on each beat change.";
  pub const STRING_HELP_TICKER_INTERVAL: &str =
    "Prints the current time on each beat interval change.";
  pub const STRING_HELP: &str = "For more, use --help";
}

#[test]
fn test_language() {
  if cfg!(any(
    feature = "eng",
    all(not(feature = "zhn"), not(feature = "pt"))
  )) {
    assert_eq!(strings::STRING_TITLE, "Chord Composer");
  } else if cfg!(feature = "pt") {
    assert_eq!(
      strings::STRING_TITLE,
      "Chord Composer (Compositor de acorde)"
    );
  } else if cfg!(feature = "zhn") {
    assert_eq!(strings::STRING_TITLE, "Chord Composer (和弦作曲家)");
  } else {
    assert!(false, "No language supplied");
  }
}
