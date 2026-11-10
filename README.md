# DuplicatiToTreeSize

Утилита командной строки для конвертации списка исключений Duplicati в XML-фильтр, который можно импортировать в
TreeSize.

Программа читает filter-файл Duplicati построчно, преобразует exclude-паттерны в формат TreeSize и записывает результат
в указанный XML-файл.

## Возможности

- поддержка входного и выходного файлов как позиционных аргументов или через `-i` / `-o`;
- пропуск пустых строк и комментариев, начинающихся с `#`;
- обработка строк с префиксом `-` как exclude-паттернов;
- предупреждение и пропуск include-паттернов с префиксом `+`;
- преобразование завершающего разделителя каталога в шаблон `\*`;
- экранирование специальных XML-символов;
- опция `--invert-negating` для управления значением `<Negated>`.

## Требования

- Rust и Cargo;
- Windows рекомендуется, поскольку пути в filter-файлах и примерах используют обратный слеш.

## Сборка

Для сборки debug-версии:

```powershell
cargo build
```

Для сборки оптимизированной версии:

```powershell
cargo build --release
```

Исполняемый файл после release-сборки находится в `target\release\DuplicatiToTreeSize.exe`.

## Использование

Позиционные аргументы:

```powershell
DuplicatiToTreeSize.exe input.txt output.xml
```

Именованные аргументы:

```powershell
DuplicatiToTreeSize.exe `
  --input-file input.txt `
  --output-file output.xml
```

С инвертированным значением `<Negated>`:

```powershell
DuplicatiToTreeSize.exe input.txt output.xml --invert-negating
```

Справка:

```powershell
DuplicatiToTreeSize.exe --help
```

## Формат входного файла

Каждая строка может содержать один паттерн:

```text
# комментарий
-D:\TEMP\Microsoft\
-D:\TEMP\*.xml
+D:\TEMP\keep
```

Строки с `-` преобразуются и попадают в выходной XML. Строки с `+` пропускаются с предупреждением, поскольку программа
предназначена для переноса именно списка исключений. Строки без префикса также рассматриваются как exclude-паттерны.

Для каталога вроде `D:\TEMP\Microsoft\` утилита формирует шаблон `D:\TEMP\Microsoft\*`. Ведущий `*` у паттерна
удаляется; существующие шаблоны с `\*` и `\*\*` не дублируются.

## Проверка

Запуск тестов:

```powershell
cargo test
```

Проверка формата и предупреждений выполняется через сообщения в stderr. При ошибке чтения входного файла или записи XML
программа завершает работу с кодом `1`.

## License

Copyright (c) 2026 MrMarvel

This work is licensed under
a [Creative Commons Attribution-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-sa/4.0/).
