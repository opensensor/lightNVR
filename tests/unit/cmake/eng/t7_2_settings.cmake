add_layer3_test(test_t7_2_settings)
target_link_options(test_t7_2_settings PRIVATE
    -Wl,--wrap=save_config -Wl,--wrap=system -Wl,--wrap=popen)
set_tests_properties(test_t7_2_settings PROPERTIES TIMEOUT 30 LABELS "engineering;security")
