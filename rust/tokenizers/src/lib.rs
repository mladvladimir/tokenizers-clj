// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jstring;

use tokenizers::tokenizer::{Result, Tokenizer, EncodeInput, Encoding};
use tokenizers::models::bpe::BPE;

fn pretrained_encode(vocab: &str, merges: &str, text: &str) -> Result<Encoding> {
    let bpe_builder = BPE::from_files(vocab, merges)?;
    let bpe = bpe_builder
        .dropout(0.1)
        // .unk_token("<unk>".into())
        .build()?;
    let mut tokenizer = Tokenizer::new(Box::new(bpe));
    let encoding = tokenizer.encode(EncodeInput::Single(text.into()))?;

    Ok(encoding)
}

#[no_mangle]
pub extern "system" fn Java_mladvladimir_clojure_rust_ClojureRust_getTokensRust(env: JNIEnv,
// This is the class that owns our static method. It's not going to be used,
// but still must be present to match the expected signature of a static
// native method.
                                                                                _class: JClass,
                                                                                vocab: JString,
                                                                                merges: JString,
                                                                                text: JString)
                                                                                -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let vocab: String =
        env.get_string(vocab).expect("Couldn't get java string!").into();
    let merges: String =
        env.get_string(merges).expect("Couldn't get java string!").into();
    let text: String =
        env.get_string(text).expect("Couldn't get java string!").into();


    let encoding = pretrained_encode(vocab.as_str(),merges.as_str(),text.as_str());
    let output = env.new_string(format!("{:?}", encoding.unwrap().get_tokens()))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_inner()

}
