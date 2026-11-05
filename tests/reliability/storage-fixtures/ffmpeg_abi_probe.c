/* Compile with the SAME include/link selection as the native qualification. */
#include <stdio.h>
#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavutil/avutil.h>

int main(void) {
    unsigned codec = avcodec_version() >> 16;
    unsigned format = avformat_version() >> 16;
    unsigned util = avutil_version() >> 16;
    printf("avcodec header=%d runtime=%u; avformat header=%d runtime=%u; "
           "avutil header=%d runtime=%u\n", LIBAVCODEC_VERSION_MAJOR, codec,
           LIBAVFORMAT_VERSION_MAJOR, format, LIBAVUTIL_VERSION_MAJOR, util);
    return codec != LIBAVCODEC_VERSION_MAJOR ||
           format != LIBAVFORMAT_VERSION_MAJOR || util != LIBAVUTIL_VERSION_MAJOR;
}
