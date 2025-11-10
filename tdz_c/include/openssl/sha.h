#ifndef OPENSSL_SHA_H
#define OPENSSL_SHA_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Digest lengths
#define SHA_DIGEST_LENGTH 20
#define SHA224_DIGEST_LENGTH 28
#define SHA256_DIGEST_LENGTH 32
#define SHA384_DIGEST_LENGTH 48
#define SHA512_DIGEST_LENGTH 64
#define SHA512_224_DIGEST_LENGTH 28
#define SHA512_256_DIGEST_LENGTH 32

// SHA-1 context
typedef struct SHAstate_st {
    uint32_t h0, h1, h2, h3, h4;
    uint32_t Nl, Nh;
    uint32_t data[16];
    unsigned int num;
} SHA_CTX;

// SHA-224 context
typedef struct SHA256state_st {
    uint32_t h[8];
    uint32_t Nl, Nh;
    uint32_t data[16];
    unsigned int num, md_len;
} SHA256_CTX;

// SHA-256 context (same as SHA-224)
// SHA256_CTX already defined above

// SHA-384 context
typedef struct SHA512state_st {
    uint64_t h[8];
    uint64_t Nl, Nh;
    union {
        uint64_t d[16];
        unsigned char p[128];
    } u;
    unsigned int num, md_len;
} SHA512_CTX;

// SHA-512 context (same as SHA-384)
// SHA512_CTX already defined above

// SHA-1 functions
int SHA1_Init(SHA_CTX* c);
int SHA1_Update(SHA_CTX* c, const void* data, size_t len);
int SHA1_Final(unsigned char* md, SHA_CTX* c);
unsigned char* SHA1(const unsigned char* d, size_t n, unsigned char* md);

// SHA-224 functions
int SHA224_Init(SHA256_CTX* c);
int SHA224_Update(SHA256_CTX* c, const void* data, size_t len);
int SHA224_Final(unsigned char* md, SHA256_CTX* c);
unsigned char* SHA224(const unsigned char* d, size_t n, unsigned char* md);

// SHA-256 functions
int SHA256_Init(SHA256_CTX* c);
int SHA256_Update(SHA256_CTX* c, const void* data, size_t len);
int SHA256_Final(unsigned char* md, SHA256_CTX* c);
unsigned char* SHA256(const unsigned char* d, size_t n, unsigned char* md);

// SHA-384 functions
int SHA384_Init(SHA512_CTX* c);
int SHA384_Update(SHA512_CTX* c, const void* data, size_t len);
int SHA384_Final(unsigned char* md, SHA512_CTX* c);
unsigned char* SHA384(const unsigned char* d, size_t n, unsigned char* md);

// SHA-512 functions
int SHA512_Init(SHA512_CTX* c);
int SHA512_Update(SHA512_CTX* c, const void* data, size_t len);
int SHA512_Final(unsigned char* md, SHA512_CTX* c);
unsigned char* SHA512(const unsigned char* d, size_t n, unsigned char* md);

// SHA-512/224 functions
int SHA512_224_Init(SHA512_CTX* c);
int SHA512_224_Update(SHA512_CTX* c, const void* data, size_t len);
int SHA512_224_Final(unsigned char* md, SHA512_CTX* c);
unsigned char* SHA512_224(const unsigned char* d, size_t n, unsigned char* md);

// SHA-512/256 functions
int SHA512_256_Init(SHA512_CTX* c);
int SHA512_256_Update(SHA512_CTX* c, const void* data, size_t len);
int SHA512_256_Final(unsigned char* md, SHA512_CTX* c);
unsigned char* SHA512_256(const unsigned char* d, size_t n, unsigned char* md);

#ifdef __cplusplus
}
#endif

#endif // OPENSSL_SHA_H
