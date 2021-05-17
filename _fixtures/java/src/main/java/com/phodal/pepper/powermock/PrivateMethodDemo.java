package com.phodal.pepper.powermock;

public class PrivateMethodDemo {
    public String say(String name) {
        return sayIt(name);
    }

    public String enhancedSay(String firstName, String lastName) {
        return sayIt(firstName, " ", lastName);
    }

    public String sayYear(String name, int years) {
        return doSayYear(years, name);
    }

    private String doSayYear(int years, String name) {
        return "Hello " + name + ", you are " + years + " old.";
    }

    private String sayIt(String firstName, String spacing, String lastName) {
        return "Hello" + firstName + spacing + lastName;
    }

    private String sayIt(String name) {
        return "Hello " + name;
    }
}
