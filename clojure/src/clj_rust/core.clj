(ns clj-rust.core
    (:require [clojure.edn :as edn])
    (:import [mladvladimir.clojure.rust ClojureRust]))

(clojure.lang.RT/loadLibrary "tokenizers")

(defn get-tokenizer [vocab merges]
  (ClojureRust/getTokenizer vocab merges))

(defn get-tokens
  [tokenizer text]
  (edn/read-string (ClojureRust/getTokens tokenizer text)))
