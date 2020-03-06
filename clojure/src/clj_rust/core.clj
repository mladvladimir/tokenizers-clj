(ns clj-rust.core
    (:require [clojure.java.io :as io])
    (:import [mladvladimir.clojure.rust ClojureRust]))

(defn init! []
  (let [home (System/getProperty "user.home")
        lib-dir (io/file home ".clojure_rust")]
      (.mkdirs lib-dir)
      (doseq [lib-name ["libtokenizers.dylib" "libtokenizers.so"]]
          (when-let [resource (io/resource lib-name)]
              (let [lib-file (io/file lib-dir lib-name)]
                  (io/copy (io/input-stream resource) lib-file))))
      (System/setProperty "java.library.path" (.getPath lib-dir))))

(clojure.lang.RT/loadLibrary "tokenizers")


(defn get-tokens
  [vocab merges text]
  (read-string (ClojureRust/getTokens vocab merges text)))