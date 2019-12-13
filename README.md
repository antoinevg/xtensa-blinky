Like this:

    # activate esp-idf environment
    export IDF_PATH=/path/to/esp-idf
    . ./$IDF_PATH/export.sh

    # activate rust-xtensa environment
    . ./setenv

    # build binary
    make build

    # flash binary
    make flash
