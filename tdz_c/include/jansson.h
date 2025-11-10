#ifndef JANSSON_H
#define JANSSON_H

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

// Forward declaration
typedef struct json_t json_t;

// Integer type (must be defined early)
typedef int64_t json_int_t;

// Callback types (must be defined before use)
typedef int (*json_dump_callback_t)(const char* buffer, size_t size, void* data);
typedef size_t (*json_load_callback_t)(char* buffer, size_t buflen, void* data);

// Error structure
typedef struct {
    int line;
    int column;
    int position;
    char source[80];
    char text[160];
} json_error_t;

// Type enumeration
typedef enum {
    JSON_OBJECT,
    JSON_ARRAY,
    JSON_STRING,
    JSON_INTEGER,
    JSON_REAL,
    JSON_TRUE,
    JSON_FALSE,
    JSON_NULL
} json_type_t;

// Encoding flags
#define JSON_INDENT(n) ((n) << 8)
#define JSON_COMPACT 0x1
#define JSON_ENSURE_ASCII 0x2
#define JSON_SORT_KEYS 0x4
#define JSON_PRESERVE_ORDER 0x8
#define JSON_ENCODE_ANY 0x10
#define JSON_ESCAPE_SLASH 0x20
#define JSON_REAL_PRECISION(n) ((n) << 11)

// Decode flags
#define JSON_REJECT_DUPLICATES 0x1
#define JSON_DISABLE_EOF_CHECK 0x2
#define JSON_DECODE_ANY 0x4
#define JSON_DECODE_INT_AS_REAL 0x8
#define JSON_ALLOW_NUL 0x10

// Object creation
json_t* json_object(void);
json_t* json_array(void);
json_t* json_string(const char* value);
json_t* json_string_nocheck(const char* value);
json_t* json_stringn(const char* value, size_t len);
json_t* json_integer(json_int_t value);
json_t* json_real(double value);
json_t* json_true(void);
json_t* json_false(void);
json_t* json_boolean(int value);
json_t* json_null(void);

// Type checking
// Note: In real jansson, these are macros, but we declare them as functions for compatibility
int json_is_object(const json_t* json);
int json_is_array(const json_t* json);
int json_is_string(const json_t* json);  // Real jansson: macro using json_typeof
int json_is_integer(const json_t* json);
int json_is_real(const json_t* json);
int json_is_number(const json_t* json);
int json_is_boolean(const json_t* json);
int json_is_true(const json_t* json);
int json_is_false(const json_t* json);
int json_is_null(const json_t* json);

// Type information
json_type_t json_typeof(const json_t* json);
const char* json_type_name(json_type_t type);

// Value accessors
const char* json_string_value(const json_t* string);
size_t json_string_length(const json_t* string);
json_int_t json_integer_value(const json_t* integer);
double json_real_value(const json_t* real);
int json_boolean_value(const json_t* boolean);

// Object operations
size_t json_object_size(const json_t* object);
json_t* json_object_get(const json_t* object, const char* key);
int json_object_set_new(json_t* object, const char* key, json_t* value);
int json_object_set(json_t* object, const char* key, json_t* value);
int json_object_set_new_nocheck(json_t* object, const char* key, json_t* value);
int json_object_set_nocheck(json_t* object, const char* key, json_t* value);
int json_object_del(json_t* object, const char* key);
int json_object_clear(json_t* object);
int json_object_update(json_t* object, json_t* other);
int json_object_update_existing(json_t* object, json_t* other);
int json_object_update_missing(json_t* object, json_t* other);
void* json_object_iter(json_t* object);
void* json_object_iter_at(json_t* object, const char* key);
void* json_object_iter_next(json_t* object, void* iter);
const char* json_object_iter_key(void* iter);
json_t* json_object_iter_value(void* iter);
int json_object_iter_set_new(json_t* object, void* iter, json_t* value);

// Array operations
size_t json_array_size(const json_t* array);
json_t* json_array_get(const json_t* array, size_t index);
int json_array_set_new(json_t* array, size_t index, json_t* value);
int json_array_set(json_t* array, size_t index, json_t* value);
int json_array_append_new(json_t* array, json_t* value);
int json_array_append(json_t* array, json_t* value);
int json_array_insert_new(json_t* array, size_t index, json_t* value);
int json_array_insert(json_t* array, size_t index, json_t* value);
int json_array_remove(json_t* array, size_t index);
int json_array_clear(json_t* array);
int json_array_extend(json_t* array, json_t* other);

// Reference counting
json_t* json_incref(json_t* json);
void json_decref(json_t* json);  // Note: In real jansson this is an inline, but we declare it as function for compatibility
int json_refcount(json_t* json);

// Equality
int json_equal(const json_t* value1, const json_t* value2);

// Copying
json_t* json_copy(const json_t* value);
json_t* json_deep_copy(const json_t* value);

// Encoding
char* json_dumps(const json_t* json, size_t flags);
int json_dumpf(const json_t* json, FILE* output, size_t flags);
int json_dump_file(const json_t* json, const char* path, size_t flags);
int json_dump_callback(const json_t* json, json_dump_callback_t callback, void* data, size_t flags);

// Decoding
json_t* json_loads(const char* input, size_t flags, json_error_t* error);
json_t* json_loadf(FILE* input, size_t flags, json_error_t* error);
json_t* json_load_file(const char* path, size_t flags, json_error_t* error);
json_t* json_load_callback(json_load_callback_t callback, void* data, size_t flags, json_error_t* error);

// Error handling
void json_error_end_of_input(json_error_t* error);
void json_error_set_source(json_error_t* error, const char* source);

// Pack/unpack
int json_pack(json_t** root, const char* fmt, ...);
int json_pack_ex(json_t** root, json_error_t* error, size_t flags, const char* fmt, ...);
int json_unpack(const json_t* root, const char* fmt, ...);
int json_unpack_ex(const json_t* root, json_error_t* error, size_t flags, const char* fmt, ...);

#ifdef __cplusplus
}
#endif

#endif // JANSSON_H
