Run `script/compile` to build the Rust lib.

Enter the `clojure` directory and start a REPL with `lein repl`.

```
(def t (get-tokenizer "roberta-base-vocab.json" "roberta-base-merges.txt")) ;; this contains a pointer to a tokenizer
(time (get-tokens t "hello there"))
"Elapsed time: 0.074647 msecs"
["hell" "ot" "here"]
```

You can get the `.json` and `.txt` files here:

```
https://s3.amazonaws.com/models.huggingface.co/bert/roberta-base-vocab.json
https://s3.amazonaws.com/models.huggingface.co/bert/roberta-base-merges.txt
```

## License
Copyright © 2020 Vladimir Mladenovic

Distributed under the EPL License, same as Clojure. See LICENSE.
