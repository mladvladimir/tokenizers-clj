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
use jni::sys::{jlong, jstring};

use tokenizers::tokenizer::{Result, Tokenizer, EncodeInput};
use tokenizers::models::bpe::BPE;

fn make_tokenizer(vocab: &str, merges: &str) -> Result<Tokenizer> {

    let bpe_builder = BPE::from_files(vocab, merges)?;
    let bpe = bpe_builder
        .dropout(0.1)
        // .unk_token("<unk>".into())
        .build()?;
    let tokenizer = Tokenizer::new(Box::new(bpe));
    Ok(tokenizer)
}

#[no_mangle]
pub unsafe extern "system" fn Java_mladvladimir_clojure_rust_ClojureRust_getTokenizerRust(env: JNIEnv,
                                                                           _class: JClass,
                                                                           vocab: JString,
                                                                           merges: JString) -> jlong {
    print!("{}","hello!");
    let vocab: String =
        env.get_string(vocab).expect("Couldn't get java string!").into();
    let merges: String =
        env.get_string(merges).expect("Couldn't get java string!").into();
    let tokenizer = make_tokenizer(&vocab, &merges).unwrap();
    Box::into_raw(Box::new(tokenizer)) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_mladvladimir_clojure_rust_ClojureRust_getTokensRust(env: JNIEnv,
// This is the class that owns our static method. It's not going to be used,
// but still must be present to match the expected signature of a static
// native method.
                                                                                _class: JClass,
                                                                                tokenizer_ptr: jlong,
                                                                                text: JString)
                                                                                -> jstring {
    let tokenizer = &*(tokenizer_ptr as *mut Tokenizer);
    let text: String =
        env.get_string(text).expect("Couldn't get java string!").into();
    let encoding = tokenizer.encode(EncodeInput::Single(text.into()));
    let output = env.new_string(format!("{:?}", encoding.unwrap().get_tokens()))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_inner()

}
