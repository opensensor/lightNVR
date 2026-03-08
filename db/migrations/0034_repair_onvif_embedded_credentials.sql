-- Repair legacy ONVIF streams that still store credentials inside url

-- migrate:up

UPDATE streams
   SET url = substr(url, 1, instr(url, '://') + 2) ||
             substr(substr(url, instr(url, '://') + 3),
                    instr(substr(url, instr(url, '://') + 3), '@') + 1),
       onvif_username = CASE
           WHEN COALESCE(onvif_username, '') = '' THEN
               CASE
                   WHEN instr(substr(substr(url, instr(url, '://') + 3),
                                     1,
                                     instr(substr(url, instr(url, '://') + 3), '@') - 1),
                              ':') > 0 THEN
                       substr(substr(substr(url, instr(url, '://') + 3),
                                           1,
                                           instr(substr(url, instr(url, '://') + 3), '@') - 1),
                              1,
                              instr(substr(substr(url, instr(url, '://') + 3),
                                                  1,
                                                  instr(substr(url, instr(url, '://') + 3), '@') - 1),
                                     ':') - 1)
                   ELSE
                       substr(substr(url, instr(url, '://') + 3),
                              1,
                              instr(substr(url, instr(url, '://') + 3), '@') - 1)
               END
           ELSE onvif_username
       END,
       onvif_password = CASE
           WHEN COALESCE(onvif_password, '') = '' THEN
               CASE
                   WHEN instr(substr(substr(url, instr(url, '://') + 3),
                                     1,
                                     instr(substr(url, instr(url, '://') + 3), '@') - 1),
                              ':') > 0 THEN
                       substr(substr(substr(url, instr(url, '://') + 3),
                                           1,
                                           instr(substr(url, instr(url, '://') + 3), '@') - 1),
                              instr(substr(substr(url, instr(url, '://') + 3),
                                                  1,
                                                  instr(substr(url, instr(url, '://') + 3), '@') - 1),
                                     ':') + 1)
                   ELSE ''
               END
           ELSE onvif_password
       END
 WHERE instr(url, '://') > 0
   AND instr(substr(url, instr(url, '://') + 3), '@') > 0
   AND (
        is_onvif = 1 OR
        COALESCE(onvif_profile, '') != '' OR
        COALESCE(onvif_username, '') != '' OR
        COALESCE(onvif_password, '') != '' OR
        COALESCE(onvif_port, 0) > 0
   );

-- migrate:down

SELECT 1;