set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR armv7)

set(CMAKE_C_COMPILER arm-linux-gnueabihf-gcc)
set(CMAKE_CXX_COMPILER arm-linux-gnueabihf-g++)
set(CMAKE_ASM_COMPILER arm-linux-gnueabihf-gcc)

# Match the linux/arm/v7 image contract. LiteRT and XNNPACK both require NEON
# and an explicit IEEE fp16 format when cross-compiling their ARMv7 kernels.
set(_LIGHTNVR_ARMV7_FLAGS
    "-march=armv7-a -mfpu=neon-vfpv4 -mfloat-abi=hard -mfp16-format=ieee")
set(CMAKE_C_FLAGS_INIT "${_LIGHTNVR_ARMV7_FLAGS}")
set(CMAKE_CXX_FLAGS_INIT "${_LIGHTNVR_ARMV7_FLAGS}")

# GNU ld does not automatically search Debian's multiarch directories for
# transitive shared-library dependencies. Mosquitto, for example, depends on
# libpicohttpparser; make those target directories available at link time
# without embedding builder paths in the resulting executable.
set(_LIGHTNVR_ARMV7_RPATH_LINK
    "-Wl,-rpath-link,/usr/lib/arm-linux-gnueabihf -Wl,-rpath-link,/lib/arm-linux-gnueabihf")
set(CMAKE_EXE_LINKER_FLAGS_INIT "${_LIGHTNVR_ARMV7_RPATH_LINK}")
set(CMAKE_SHARED_LINKER_FLAGS_INIT "${_LIGHTNVR_ARMV7_RPATH_LINK}")

# LiteRT generates FlatBuffer schemas during its target build. This binary is
# built natively in the Docker host-tools layer and must not be cross-compiled.
set(TFLITE_HOST_TOOLS_DIR "/opt/host-tools" CACHE PATH "LiteRT host tools")

# Programs such as flatc must execute on the x86_64 build host. Libraries and
# headers are resolved through Debian's arm-linux-gnueabihf multiarch paths.
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE BOTH)
set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE ONLY)
