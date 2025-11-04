# bank_record_converter
Библиотека (crate) для парсинга/сериализации/десериализации финансовых данных в несколько форматов и отдельные исполняемые cli (консольное приложение) crate, использующий данную библиотеку.


Поддерживаемые форматы:    
- [YPBankCsv](tests/data/Format_Specification/YPBankCsvFormat_ru.md) - Таблица банковских операций.

- [YPBankText](tests/data/Format_Specification/YPBankTextFromat_ru.md) - Текстовый формат описания списка операций.

- [YPBankBin](tests/data/Format_Specification/YPBankBinFormat_ru.md) - Бинарное предоставление списка операций.


Реализованны два cli инструмента:
- [ypbank_compare](examples/ypbank_compare/README.md) - сравнение двух фалов 
- [ypbank_converter](examples/ypbank_converter/README.md) - ковертация одного типа файла в другой 