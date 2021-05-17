package com.phodal.pepper.normal.date.mockito;

import java.util.Date;

interface DateTime {
    Date getDate();
}

class DateTimeImpl implements DateTime {
    @Override
    public Date getDate() {
        return new Date();
    }
}
