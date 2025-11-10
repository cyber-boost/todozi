#ifndef JSON_C_JSON_H
#define JSON_C_JSON_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Forward declaration
typedef struct json_object json_object;

// JSON type enumeration
typedef enum {
    json_type_null,
    json_type_boolean,
    json_type_double,
    json_type_int,
    json_type_object,
    json_type_array,
    json_type_string
} json_type;

// String conversion flags
#define JSON_C_TO_STRING_PLAIN 0x0
#define JSON_C_TO_STRING_PRETTY 0x1
#define JSON_C_TO_STRING_SPACED 0x2
#define JSON_C_TO_STRING_PRETTY_TAB 0x4

// Object creation functions
json_object* json_object_new_object(void);
json_object* json_object_new_array(void);
json_object* json_object_new_string(const char* s);
json_object* json_object_new_string_len(const char* s, int len);
json_object* json_object_new_int(int32_t i);
json_object* json_object_new_int64(int64_t i);
json_object* json_object_new_uint64(uint64_t i);
json_object* json_object_new_double(double d);
json_object* json_object_new_boolean(int b);

// Object manipulation
void json_object_put(json_object* obj);
int json_object_get_userdata(json_object* obj, void** userdata);
int json_object_is_type(const json_object* obj, json_type type);
json_type json_object_get_type(const json_object* obj);

// Object property access
void json_object_object_add(json_object* obj, const char* key, json_object* val);
int json_object_object_add_ex(json_object* obj, const char* key, json_object* val, unsigned int opts);
int json_object_object_get_ex(const json_object* obj, const char* key, json_object** value);
json_object* json_object_object_get(const json_object* obj, const char* key);
int json_object_object_del(json_object* obj, const char* key);

// Value getters
const char* json_object_get_string(const json_object* obj);
int json_object_get_string_len(const json_object* obj);
int32_t json_object_get_int(const json_object* obj);
int64_t json_object_get_int64(const json_object* obj);
uint64_t json_object_get_uint64(const json_object* obj);
double json_object_get_double(const json_object* obj);
int json_object_get_boolean(const json_object* obj);

// Array operations
int json_object_array_add(json_object* obj, json_object* val);
int json_object_array_put_idx(json_object* obj, size_t idx, json_object* val);
json_object* json_object_array_get_idx(const json_object* obj, size_t idx);
size_t json_object_array_length(const json_object* obj);
int json_object_array_del_idx(json_object* obj, size_t idx, size_t count);

// Tokener error codes (must be defined before use)
enum json_tokener_error {
    json_tokener_success,
    json_tokener_continue,
    json_tokener_error_depth,
    json_tokener_error_size,
    json_tokener_error_inf_or_nan,
    json_tokener_error_parse_unexpected,
    json_tokener_error_parse_null,
    json_tokener_error_parse_boolean,
    json_tokener_error_parse_number,
    json_tokener_error_parse_array,
    json_tokener_error_parse_object_key_name,
    json_tokener_error_parse_object_key_sep,
    json_tokener_error_parse_object_value_sep,
    json_tokener_error_parse_string,
    json_tokener_error_parse_comment,
    json_tokener_error_parse_eof
};

// String conversion
const char* json_object_to_json_string(const json_object* obj);
const char* json_object_to_json_string_ext(const json_object* obj, int flags);
const char* json_object_to_json_string_length(const json_object* obj, int flags, size_t* length);

// Parsing
json_object* json_tokener_parse(const char* str);
json_object* json_tokener_parse_verbose(const char* str, enum json_tokener_error* error);

// Utility functions
int json_object_equal(const json_object* obj1, const json_object* obj2);
size_t json_object_size(const json_object* obj);
int json_object_shallow_copy(json_object* src, json_object* dst);

#ifdef __cplusplus
}
#endif

#endif // JSON_C_JSON_H
