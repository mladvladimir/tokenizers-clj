Run `script/compile` to build the Rust lib.

From repl:

```
clj-rust.core=> (init!)
clj-rust.core=> (clojure.lang.RT/loadLibrary "tokenizers")
clj-rust.core=> (get-tokens
                    "./path/to/vocab.json"
                    "./path/to/merges.txt"
                    "piece of text")
```

You can get the `.json` and `.txt` files here:

```
https://s3.amazonaws.com/models.huggingface.co/bert/roberta-base-vocab.json
https://s3.amazonaws.com/models.huggingface.co/bert/roberta-base-merges.txt
```

## License
Copyright © 2020 Vladimir Mladenovic

Distributed under the EPL License, same as Clojure. See LICENSE.
