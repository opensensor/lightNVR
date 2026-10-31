add_layer3_test(test_t7_25_retention)
target_link_options(test_t7_25_retention PRIVATE -Wl,--wrap=stat -Wl,--wrap=fstat
    -Wl,--wrap=openat -Wl,--wrap=close -Wl,--wrap=opendir -Wl,--wrap=closedir
    -Wl,--wrap=readdir -Wl,--wrap=realloc -Wl,--wrap=strdup
    -Wl,--wrap=syscall -Wl,--wrap=unlinkat)
set_tests_properties(test_t7_25_retention PROPERTIES TIMEOUT 30 LABELS "engineering;security")
