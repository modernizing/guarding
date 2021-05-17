package com.phodal.pepper.powermock;

import java.io.IOException;
import java.io.UnsupportedEncodingException;
import java.net.*;
import java.util.Collections;
import java.util.List;
import java.util.UUID;

public class SystemClassUser {
    public void threadSleep() throws InterruptedException {
        Thread.sleep(5000);
    }

    public String performEncode() throws UnsupportedEncodingException {
        return URLEncoder.encode("string", "utf8");
    }

    public Process executeCommand() throws IOException {
        return Runtime.getRuntime().exec("command");
    }

    public String getSystemProperty() throws IOException {
        return System.getProperty("property");
    }

    public void doMoreComplicatedStuff() throws IOException {
        System.setProperty("nanoTime", Long.toString(System.nanoTime()));
    }

    public void copyProperty(String to, String from) throws IOException {
        System.setProperty(to, System.getProperty(from));
    }

    public String format(String one, String args) throws IOException {
        return String.format(one, args);
    }

    public URL newURL(String anUrl) throws MalformedURLException {
        return new URL(anUrl);
    }

    public StringBuilder newStringBuilder() {
        return new StringBuilder();
    }

    public void shuffleCollection(List<?> list) {
        Collections.shuffle(list);
    }

    public URLConnection useURL(URL url) throws IOException {
        return url.openConnection();
    }

    public InetAddress getLocalHost() throws IOException {
        return InetAddress.getLocalHost();
    }

    public String generatePerishableToken() {
        final UUID uuid = UUID.randomUUID();
        final String toString = uuid.toString();
        final String result = toString.replaceAll("-", "");
        return result;
    }

    public int lengthOf(StringBuilder to){
        // The trick here is the casting to CharSequence,
        // this is to test that the runtime type(StringBuilder) is checked for mocked calls and not
        // the compile-time type (CharSequence)
        return lengthOf((CharSequence) to);
    }

    private int lengthOf(CharSequence to){
        return to.length();
    }
}
