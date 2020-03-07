package mladvladimir.clojure.rust;

public class ClojureRust {

    private static native long getTokenizerRust(String vocab, String merges);
    private static native String getTokensRust(long tokenizerPtr, String text);

    public static long getTokenizer(String vocab, String merges) throws java.io.IOException {
        return getTokenizerRust(vocab, merges);
    }

    public static String getTokens(long tokenizerPtr, String text) throws java.io.IOException {
        String output = getTokensRust(tokenizerPtr, text);
        return output;
    }
}
