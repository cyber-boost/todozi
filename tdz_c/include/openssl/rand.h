#ifndef OPENSSL_RAND_H
#define OPENSSL_RAND_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Random number generation
int RAND_bytes(unsigned char* buf, int num);
int RAND_priv_bytes(unsigned char* buf, int num);
int RAND_pseudo_bytes(unsigned char* buf, int num);

// Status and seeding
int RAND_status(void);
void RAND_seed(const void* buf, int num);
void RAND_add(const void* buf, int num, double entropy);
int RAND_load_file(const char* file, long max_bytes);
int RAND_write_file(const char* file);
const char* RAND_file_name(char* file, size_t num);

// Error handling
const void* RAND_get_rand_method(void);
void RAND_set_rand_method(const void* meth);
void RAND_cleanup(void);

// Platform-specific
void RAND_screen(void);
int RAND_poll(void);

// Deprecated functions (for compatibility)
void RAND_keep_random_devices_open(int keep);

#ifdef __cplusplus
}
#endif

#endif // OPENSSL_RAND_H
