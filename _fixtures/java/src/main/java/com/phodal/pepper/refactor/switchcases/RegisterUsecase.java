package com.phodal.pepper.refactor.switchcases;

import com.phodal.pepper.refactor.switchcases.clz.CaseInterface;
import org.reflections.Reflections;

import java.util.HashMap;

public class RegisterUsecase {
    HashMap<String, String> caseMaps = new HashMap<String, String>();

    public void registerCase() {
        Reflections ref = new Reflections("com.phodal.pepper.refactor.switchcases");
        for (Class<?> cl : ref.getTypesAnnotatedWith(RegisterPattern.class)) {
            RegisterPattern findable = cl.getAnnotation(RegisterPattern.class);
            String className = cl.getName();
            String registerName = findable.register();
            caseMaps.put(registerName, className);
        }
    }

    public void runCase(String caseName) {
        String className = caseMaps.get(caseName);
        try {
            Class<?> DemoClass = Class.forName(className);
            CaseInterface caseInterface = (CaseInterface) DemoClass.newInstance();
            caseInterface.buildMap();
        } catch (ClassNotFoundException e) {
            e.printStackTrace();
        } catch (IllegalAccessException e) {
            e.printStackTrace();
        } catch (InstantiationException e) {
            e.printStackTrace();
        }
    }
}
