package com.phodal.pepper.refactor.staticclass;

import java.util.HashMap;
import java.util.Map;

public class LogFileHandle {
    public static void verifyFilesBySuffix(String suffix) {
        HashMap<String, String> files = getFilesMap();
        for (Map.Entry<String, String> entry : files.entrySet()) {
            if (entry.getValue().endsWith(suffix)) {
                FileHandler.verify(entry.getValue());
            }
        }
    }
//
//    public static void verifyFilesBySuffix(String suffix) {
//        verifyFilesBySuffix(suffix, new FileVerifyFactoryImpl());
//    }
//
//    private static void verifyFilesBySuffix(String suffix, FileVerifyFactoryImpl fileVerifyFactoryImpl) {
//        HashMap<String, String> files = getFilesMap();
//        for (Map.Entry<String, String> entry : files.entrySet()) {
//            if (entry.getValue().endsWith(suffix)) {
//                FileVerify fileVerify = fileVerifyFactoryImpl.genFileVerify();
//                fileVerify.getVerify(entry);
//            }
//        }
//    }

    private static HashMap<String, String> getFilesMap() {
        HashMap<String, String> filesMap = new HashMap<>();
        filesMap.put("home", "/home");
        filesMap.put("about", "/pages/about");
        return filesMap;
    }
}
