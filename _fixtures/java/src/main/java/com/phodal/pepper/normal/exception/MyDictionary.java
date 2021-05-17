package com.phodal.pepper.normal.exception;

import java.util.HashMap;
import java.util.Map;

public class MyDictionary {
    private Map<String, String> wordMap = new HashMap<>();

    public void add(String word, String meaning) {
        wordMap.put(word, meaning);
    }

    public String getMeaning(String word) {
        return wordMap.get(word);
    }
}
