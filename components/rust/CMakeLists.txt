idf_component_register()

# add any additional system libraries required to to last line, e.g. c
# m gcc following rust_main
target_link_libraries(${COMPONENT_LIB} INTERFACE "-L ${CMAKE_CURRENT_SOURCE_DIR}/target/xtensa-esp32-none-elf/release")
target_link_libraries(${COMPONENT_LIB} INTERFACE rust_main)

externalproject_add(rust_build
  PREFIX ${COMPONENT_DIR}
  SOURCE_DIR ${COMPONENT_DIR}/src
  CONFIGURE_COMMAND ""
  BUILD_IN_SOURCE 0
  BUILD_COMMAND rustup run xtensa xargo build --target xtensa-esp32-none-elf --release
  INSTALL_COMMAND ""
  )
