# Events and errors

Events mode is `EventsMode78` (CEP-47 / CES / Native / NativeBytes). Set at install or upgrade.

User errors are small integers (`1..=180`). `Cep78Error` maps a subset (`PermissionDenied`, `UnknownTokenId`, …). Execution failures surface as `CepError::Execution` with optional `user_error` code.
