package com.phodal.pepper.normal.date.mockito;

public class MyDateClass {
    private final DateTime dateTime;

    public MyDateClass(final DateTime dateTime) {
        this.dateTime = dateTime;
    }

    public long getDoubleTime() {
        return dateTime.getDate().getTime() * 2;
    }
}
