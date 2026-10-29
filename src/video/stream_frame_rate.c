#include "video/stream_frame_rate.h"

#include <math.h>

static bool valid_rate(double fps) {
    return isfinite(fps) && fps > 0.0 && fps <= STREAM_FRAME_RATE_MAX;
}

static double rational_rate(AVRational rate) {
    if (rate.num <= 0 || rate.den <= 0) return 0.0;
    double fps = av_q2d(rate);
    return valid_rate(fps) ? fps : 0.0;
}

double stream_frame_rate_select(AVRational average, AVRational nominal,
                                bool *provisional) {
    double fps = rational_rate(average);
    if (provisional) *provisional = (fps == 0.0);
    return fps > 0.0 ? fps : rational_rate(nominal);
}

int stream_frame_rate_round(double fps) {
    if (!valid_rate(fps)) return 0;
    int rounded = (int)(fps + 0.5);
    return rounded > 0 ? rounded : 1;
}

int stream_frame_rate_observe(stream_frame_rate_measurement_t *m,
                              int64_t now_us) {
    if (m->fps > 0) return m->fps;
    if (!m->started) {
        m->started = true;
        m->warmup_until_us = now_us + 2000000;
    }
    if (now_us < m->warmup_until_us) return 0;
    if (m->packets++ == 0) m->window_start_us = now_us;
    int64_t elapsed_us = now_us - m->window_start_us;
    if (elapsed_us < 5000000) return 0;

    /* N arrivals span N-1 intervals. Validate before converting to int. */
    m->fps = stream_frame_rate_round((m->packets - 1) * 1000000.0 / elapsed_us);
    if (m->fps == 0) {
        /* A buffered burst is not a usable measurement; try a fresh window. */
        m->packets = 0;
    }
    return m->fps;
}
