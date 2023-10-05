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
    ]
}
```

### Running script
...
