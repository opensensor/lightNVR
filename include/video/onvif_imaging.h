#ifndef ONVIF_IMAGING_H
#define ONVIF_IMAGING_H

#include <stdbool.h>
#include <stddef.h>

typedef struct {
    bool present;
    float value;
} onvif_imaging_float_t;

typedef struct {
    bool present;
    char mode[16];
    bool has_level;
    float level;
} onvif_imaging_mode_level_t;

typedef struct {
    char video_source_token[64];
    onvif_imaging_float_t brightness;
    onvif_imaging_float_t color_saturation;
    onvif_imaging_float_t contrast;
    onvif_imaging_float_t sharpness;
    onvif_imaging_float_t noise_reduction;
    onvif_imaging_mode_level_t backlight_compensation;
    onvif_imaging_mode_level_t wide_dynamic_range;
    onvif_imaging_mode_level_t tone_compensation;
    onvif_imaging_mode_level_t defogging;
    bool has_ir_cut_filter;
    char ir_cut_filter[16];
} onvif_imaging_settings_t;

typedef struct {
    bool present;
    float min;
    float max;
} onvif_imaging_float_range_t;

typedef struct {
    bool present;
    char modes[4][16];
    int mode_count;
    onvif_imaging_float_range_t level;
} onvif_imaging_mode_level_options_t;

typedef struct {
    char video_source_token[64];
    onvif_imaging_float_range_t brightness;
    onvif_imaging_float_range_t color_saturation;
    onvif_imaging_float_range_t contrast;
    onvif_imaging_float_range_t sharpness;
    onvif_imaging_float_range_t noise_reduction;
    onvif_imaging_mode_level_options_t backlight_compensation;
    onvif_imaging_mode_level_options_t wide_dynamic_range;
    onvif_imaging_mode_level_options_t tone_compensation;
    onvif_imaging_mode_level_options_t defogging;
    char ir_cut_filter_modes[4][16];
    int ir_cut_filter_mode_count;
} onvif_imaging_options_t;

int onvif_imaging_get_settings(const char *imaging_url,
                               const char *video_source_token,
                               const char *username,
                               const char *password,
                               onvif_imaging_settings_t *settings);

int onvif_imaging_get_options(const char *imaging_url,
                              const char *video_source_token,
                              const char *username,
                              const char *password,
                              onvif_imaging_options_t *options);

int onvif_imaging_set_settings(const char *imaging_url,
                               const char *video_source_token,
                               const char *username,
                               const char *password,
                               const onvif_imaging_settings_t *settings,
                               bool force_persistence);

#endif /* ONVIF_IMAGING_H */
