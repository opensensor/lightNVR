add_layer2_test(test_t7_54)
target_link_options(test_t7_54 PRIVATE
    -Wl,--wrap=malloc -Wl,--wrap=strdup -Wl,--wrap=ftell -Wl,--wrap=fread)
set_tests_properties(test_t7_54 PROPERTIES TIMEOUT 30 LABELS "engineering;security")
