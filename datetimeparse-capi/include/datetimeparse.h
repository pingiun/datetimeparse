#ifndef _DATETIMEPARSE_H
#define _DATETIMEPARSE_H

#ifdef __cplusplus
extern "C" {
#endif

#define PDT_SUCCESS 0
#define PDT_PARSE_ERROR 1
#define PDT_MALFORMED_STR 2

struct pdt_precise_local_date_time {
    int year;
    int month;
    int day;
    int hour;
    int minute;
    int second;
    int millisecond;
};

int pdt_parse_rfc3339_datetime(const char* inp, int inp_len, struct pdt_precise_local_date_time *out);

void pdt_perror(const char* s, int error);

#ifdef __cplusplus
}
#endif

#endif
