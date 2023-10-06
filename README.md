# yandex-translate-api
Yandex Translate API with rust(based on reqwest crate).

### HTTP-GET
```
POST https://translate.api.cloud.yandex.net/translate/v2/translate
```
### Parameters in the request body
```json
{
    "sourceLanguageCode": "string",
    "targetLanguageCode": "string",
    "texts": [
        "string"
    ],
    "folderId": "<folder_id>"
}
```
### Parameters in the request headers
```json
{
    "Content-Type": "application/json",
    "Authorization": "<iam_token>"
}
```

### Dependencies
* reqwest = { version = "0.11.22", features = ["json"] }
* serde_json = "1.0.107"
* tokio = { version = "1.0.0", features = ["rt", "rt-multi-thread", "macros"] }

### Running script
#### Using cargo
```bash
cargo run <source language> <target language> <text>
```
#### Example
```bash
~$ cargo run en ru cat
"кошка"
```

