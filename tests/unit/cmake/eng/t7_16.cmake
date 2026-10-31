add_layer3_test(test_t7_16)
target_link_options(test_t7_16 PRIVATE
    -Wl,--wrap=avcodec_open2
    -Wl,--wrap=faac_adapter_open
    -Wl,--wrap=faac_adapter_close
    -Wl,--wrap=faac_adapter_get_sample_rate
    -Wl,--wrap=faac_adapter_get_frame_samples
    -Wl,--wrap=faac_adapter_get_encoder_delay
    -Wl,--wrap=faac_adapter_get_object_type
    -Wl,--wrap=faac_adapter_get_asc)
set_tests_properties(test_t7_16 PROPERTIES TIMEOUT 30 LABELS "engineering;media")
