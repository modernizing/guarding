package com.phodal.pepper.refactor.switchcases.clz;

import com.phodal.pepper.refactor.switchcases.RegisterPattern;

@RegisterPattern(register = "CASEB")
public class CaseB implements CaseInterface {

    @Override
    public void buildMap() {
        System.out.println("CaseB - buildMap");
    }
}
