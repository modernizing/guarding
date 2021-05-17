package com.phodal.pepper.powermock;

public class ClassCallStaticMethodObj {
    public void execute() {
        boolean foo = StaticMethod.firstStaticMethod("2");
        int bar = StaticMethod.secondStaticMethod();
    }
}
