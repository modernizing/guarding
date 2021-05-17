package com.phodal.pepper.powermock;

import java.util.Calendar;
import java.util.Date;

public class MockCalenderInstance {
    public Date getDate() {
        return Calendar.getInstance().getTime();
    }
}
