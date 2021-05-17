package com.phodal.pepper.refactor.switchcases.clz;

import com.phodal.pepper.refactor.switchcases.RegisterPattern;

@RegisterPattern(register = "CASEA")
public class CaseA implements CaseInterface {

    @Override
    public void buildMap() {
        System.out.println("CaseA - buildMap");
    }
}
