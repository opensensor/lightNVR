if(ENABLE_GO2RTC)
    add_layer3_test(test_t7_2_buffers)
    target_link_options(test_t7_2_buffers PRIVATE
        -Wl,--wrap=get_db_handle
        -Wl,--wrap=malloc
        -Wl,--wrap=curl_easy_perform
        -Wl,--wrap=curl_easy_getinfo)
    set_tests_properties(test_t7_2_buffers PROPERTIES TIMEOUT 30 LABELS "engineering;security")
endif()
