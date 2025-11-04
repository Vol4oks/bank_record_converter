## Описание
CLI-приложение читает данные из файла и выводить результат в stdout.

## Команда для запуска примера:
`cargo run -- --input ../../tests/data/examples_file/records_example.bin -I bin -O txt > output_file.txt`
```
ypbank_converter \
  --input <input_file> \
  --input_format <format> \
  --output_format <format> \
  > output_file.txt
```