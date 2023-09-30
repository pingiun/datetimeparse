#include <stdio.h>

#include "datetimeparse.h"

int main() {
    struct pdt_precise_local_date_time dt = {0};
    int ret = pdt_parse_rfc3339_datetime("2020-01-02a03:04:05.67891011Z", 29, &dt);
    if (ret != PDT_SUCCESS) {
        pdt_perror("pdt_parse_rfc3339_datetime", ret);
        return 1;
    }
    printf("year: %d\n", dt.year);
    printf("month: %d\n", dt.month);
    printf("day: %d\n", dt.day);
    printf("hour: %d\n", dt.hour);
    printf("minute: %d\n", dt.minute);
    printf("second: %d\n", dt.second);
    printf("millisecond: %d\n", dt.millisecond);
    return 0;
}
