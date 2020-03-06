package mladvladimir.clojure.rust;

public class ClojureRust {

    private static native String getTokensRust(String vocab, String merges, String text);

    public static String getTokens(String vocab, String merges, String text) throws java.io.IOException {
        String output = getTokensRust(vocab, merges, text);
        return output;
    }
}