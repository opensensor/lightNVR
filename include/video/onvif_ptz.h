#ifndef ONVIF_PTZ_H
#define ONVIF_PTZ_H

#include <stdbool.h>
#include <stddef.h>

/**
 * PTZ preset information
 */
typedef struct {
    char token[64];      // Preset token
    char name[128];      // Preset name
    float pan;           // Pan position (-1.0 to 1.0)
    float tilt;          // Tilt position (-1.0 to 1.0)
    float zoom;          // Zoom position (0.0 to 1.0)
} onvif_ptz_preset_t;

/**
 * PTZ capabilities
 */
typedef struct {
    bool has_continuous_move;
    bool has_absolute_move;
    bool has_relative_move;
    bool has_home_position;
    bool has_presets;
    int max_presets;
    float pan_min;
    float pan_max;
    float tilt_min;
    float tilt_max;
    float zoom_min;
    float zoom_max;
} onvif_ptz_capabilities_t;

/**
 * Get PTZ service URL from device
 * 
 * @param device_url Device service URL
 * @param username Username for authentication (can be NULL)
 * @param password Password for authentication (can be NULL)
 * @param ptz_url Buffer to fill with PTZ service URL
 * @param url_size Size of the ptz_url buffer
 * @return 0 on success, non-zero on failure
 */
int onvif_ptz_get_service_url(const char *device_url, const char *username,
                              const char *password, char *ptz_url, size_t url_size);

/**
 * Get PTZ capabilities for a profile
 * 
 * @param ptz_url PTZ service URL
 * @param profile_token Profile token
 * @param username Username for authentication (can be NULL)
 * @param password Password for authentication (can be NULL)
 * @param capabilities Buffer to fill with capabilities
 * @return 0 on success, non-zero on failure
 */
int onvif_ptz_get_capabilities(const char *ptz_url, const char *profile_token,
                               const char *username, const char *password,
                               onvif_ptz_capabilities_t *capabilities);

/**
 * Continuous move (pan/tilt/zoom at specified velocity)
 * 
 * @param ptz_url PTZ service URL
 * @param profile_token Profile token
 * @param username Username for authentication (can be NULL)
 * @param password Password for authentication (can be NULL)
 * @param pan_velocity Pan velocity (-1.0 to 1.0, 0 = stop)
 * @param tilt_velocity Tilt velocity (-1.0 to 1.0, 0 = stop)
 * @param zoom_velocity Zoom velocity (-1.0 to 1.0, 0 = stop)
 * @return 0 on success, non-zero on failure
 */
int onvif_ptz_continuous_move(const char *ptz_url, const char *profile_token,
                              const char *username, const char *password,
                              float pan_velocity, float tilt_velocity, float zoom_velocity);

/**
 * Stop PTZ movement
 * 
 * @param ptz_url PTZ service URL
 * @param profile_token Profile token
 * @param username Username for authentication (can be NULL)
 * @param password Password for authentication (can be NULL)
 * @param stop_pan_tilt Stop pan/tilt movement
 * @param stop_zoom Stop zoom movement
 * @return 0 on success, non-zero on failure
 */
int onvif_ptz_stop(const char *ptz_url, const char *profile_token,
                   const char *username, const char *password,
                   bool stop_pan_tilt, bool stop_zoom);

/**
 * Absolute move to specified position
 * 
 * @param ptz_url PTZ service URL
 * @param profile_token Profile token
 * @param username Username for authentication (can be NULL)
 * @param password Password for authentication (can be NULL)
 * @param pan Pan position (-1.0 to 1.0)
 * @param tilt Tilt position (-1.0 to 1.0)
 * @param zoom Zoom position (0.0 to 1.0)
 * @return 0 on success, non-zero on failure
 */
int onvif_ptz_absolute_move(const char *ptz_url, const char *profile_token,
                            const char *username, const char *password,
                            float pan, float tilt, float zoom);

/**
 * Relative move by specified amount
 * 
 * @param ptz_url PTZ service URL
 * @param profile_token Profile token
 * @param username Username for authentication (can be NULL)
 * @param password Password for authentication (can be NULL)
 * @param pan_delta Pan delta (-1.0 to 1.0)
 * @param tilt_delta Tilt delta (-1.0 to 1.0)
 * @param zoom_delta Zoom delta (-1.0 to 1.0)
 * @return 0 on success, non-zero on failure
 */
int onvif_ptz_relative_move(const char *ptz_url, const char *profile_token,
                            const char *username, const char *password,
                            float pan_delta, float tilt_delta, float zoom_delta);

/**
 * Go to home position
 * 
 * @param ptz_url PTZ service URL
 * @param profile_token Profile token
 * @param username Username for authentication (can be NULL)
 * @param password Password for authentication (can be NULL)
 * @return 0 on success, non-zero on failure
 */
int onvif_ptz_goto_home(const char *ptz_url, const char *profile_token,
                        const char *username, const char *password);

/**
 * Set home position to current position
 * 
 * @param ptz_url PTZ service URL
 * @param profile_token Profile token
 * @param username Username for authentication (can be NULL)
 * @param password Password for authentication (can be NULL)
 * @return 0 on success, non-zero on failure
 */
int onvif_ptz_set_home(const char *ptz_url, const char *profile_token,
                       const char *username, const char *password);

/**
 * Get PTZ presets
 */
int onvif_ptz_get_presets(const char *ptz_url, const char *profile_token,
                          const char *username, const char *password,
                          onvif_ptz_preset_t *presets, int max_presets);

/**
 * Go to preset
 */
int onvif_ptz_goto_preset(const char *ptz_url, const char *profile_token,
                          const char *username, const char *password,
                          const char *preset_token);

/**
 * Set/create preset at current position
 */
int onvif_ptz_set_preset(const char *ptz_url, const char *profile_token,
                         const char *username, const char *password,
                         const char *preset_name, char *preset_token, size_t token_size);

#endif /* ONVIF_PTZ_H */

