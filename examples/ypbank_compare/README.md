## Описание 
CLI Comparer читат данные о транзакциях из двух файлов и сравнивать их. Входные файлы могут быть в любых форматах, которые поддерживаются парсерами из lib-крейта. В случае несовпадения, утилита сообщает какая транзакция не совпала.

## Команда для запуска примера 
```
ypbank_compare --file1 records_example.bin --format1 binary --file2 records_example.csv --format2 csv
# Output: The transaction records in 'records_example.bin' and 'records_example.csv' are identical.
```