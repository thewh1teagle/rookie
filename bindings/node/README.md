# @rookie-rs/api

Extract cookies from web browsers
Bindings for [rookie](https://github.com/thewh1teagle/rookie)

## Usage

```typescript
import { chrome } from "@rookie-rs/api";

const cookies = chrome();
for (const cookie of cookies) {
  console.log(cookie);
}
```
