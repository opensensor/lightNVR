if(ENABLE_MQTT AND MOSQUITTO_FOUND)
    add_executable(test_mqtt_destination_client
        ${CMAKE_CURRENT_SOURCE_DIR}/test_mqtt_destination_client.c
        ${CMAKE_SOURCE_DIR}/src/core/mqtt_destination_client.c
    )
    target_include_directories(test_mqtt_destination_client PRIVATE
        ${CMAKE_SOURCE_DIR}/include
        ${MOSQUITTO_INCLUDE_DIRS}
    )
    target_compile_definitions(test_mqtt_destination_client PRIVATE
        ENABLE_MQTT=1
        MANAGED_PRESENCE_INTERVAL_SECONDS=1
    )
    target_link_libraries(test_mqtt_destination_client
        unity_lib
        pthread
        ${MOSQUITTO_LIBRARIES}
    )
    if(MOSQUITTO_LIBRARY_DIRS)
        target_link_directories(test_mqtt_destination_client PRIVATE
            ${MOSQUITTO_LIBRARY_DIRS}
        )
    endif()
    set_target_properties(test_mqtt_destination_client PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
    )
    add_test(NAME test_mqtt_destination_client COMMAND test_mqtt_destination_client)
    target_link_options(test_mqtt_destination_client PRIVATE
        -Wl,--wrap=db_event_destination_validate
        -Wl,--wrap=db_event_destination_get_password
        -Wl,--wrap=event_identity_get_source
        -Wl,--wrap=system_health_evaluator_service_copy_run
        -Wl,--wrap=system_health_snapshot_copy
        -Wl,--wrap=system_health_evaluator_service_active_copy
        -Wl,--wrap=_log_message_ctx
        -Wl,--wrap=lightnvr_uuid_is_valid
        -Wl,--wrap=secure_zero_memory
        -Wl,--wrap=mosquitto_new
        -Wl,--wrap=mosquitto_destroy
        -Wl,--wrap=mosquitto_connect_callback_set
        -Wl,--wrap=mosquitto_disconnect_callback_set
        -Wl,--wrap=mosquitto_publish_callback_set
        -Wl,--wrap=mosquitto_log_callback_set
        -Wl,--wrap=mosquitto_username_pw_set
        -Wl,--wrap=mosquitto_int_option
        -Wl,--wrap=mosquitto_tls_set
        -Wl,--wrap=mosquitto_tls_opts_set
        -Wl,--wrap=mosquitto_reconnect_delay_set
        -Wl,--wrap=mosquitto_will_set
        -Wl,--wrap=mosquitto_connect_async
        -Wl,--wrap=mosquitto_loop_start
        -Wl,--wrap=mosquitto_disconnect
        -Wl,--wrap=mosquitto_loop_stop
        -Wl,--wrap=mosquitto_publish
        -Wl,--wrap=mosquitto_connack_string
        -Wl,--wrap=mosquitto_strerror
    )
endif()
