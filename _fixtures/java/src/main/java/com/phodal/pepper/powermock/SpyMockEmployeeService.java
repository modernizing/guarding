package com.phodal.pepper.powermock;

public class SpyMockEmployeeService {
    public void foo() {
        log();
    }

    public void foo(String str) {
        log();
    }

    public void log() {
        System.out.println("I am console log");
    }

    public boolean exist(String name) {
        return checkExist(name);
    }

    private boolean checkExist(String name) {
        throw new UnsupportedOperationException();
    }
}
