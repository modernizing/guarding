package com.phodal.pepper.refactor.staticclass;

import java.util.Map;

public class FileVerify {
    void getVerify(Map.Entry<String, String> entry) {
        FileHandler.verify(entry.getValue());
    }
}
