#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

extern "C" {

void hello_from_rust_rs();

void open_file_rs(const char *filename_arg);

void print_area_rs(const char *filename_arg);

} // extern "C"
