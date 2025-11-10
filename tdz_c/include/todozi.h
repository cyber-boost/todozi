#ifndef TODOZI_H
#define TODOZI_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Export macro
#ifdef _WIN32
    #define TODOZI __declspec(dllexport)
#else
    #define TODOZI __attribute__((visibility("default")))
#endif

// Error handling
typedef struct {
    int32_t code;
    char* message;
} todozi_error_t;

// Opaque types
typedef struct todozi_todozi_t todozi_todozi_t;
typedef struct todozi_task_t todozi_task_t;
typedef struct todozi_memory_t todozi_memory_t;
typedef struct todozi_idea_t todozi_idea_t;
typedef struct todozi_queue_item_t todozi_queue_item_t;
typedef struct todozi_project_t todozi_project_t;
typedef struct TODOZI_key_t TODOZI_key_t;

// Array wrapper
typedef struct {
    void* data;
    size_t length;
    size_t capacity;
} todozi_array_t;

// Initialization and cleanup
TODOZI todozi_todozi_t* todozi_new();
TODOZI void todozi_free(todozi_todozi_t* instance);

// Memory management
TODOZI void todozi_string_free(char* str);
TODOZI void todozi_array_free(todozi_array_t* array);
TODOZI void todozi_error_free(todozi_error_t* error);

// Function declarations
// activate_api_key
TODOZI void todozi_activate_api_key(const char* user_id);

// activate_key
TODOZI bool todozi_activate_key(const char* user_id, todozi_error_t** error);

// activate_reminder
TODOZI void todozi_activate_reminder(const char* reminder_id);

// active
TODOZI void* todozi_active(todozi_error_t** error);

// active_percentage
TODOZI double todozi_active_percentage(todozi_error_t** error);

// add
TODOZI const char* todozi_add(const char* task_name, const char* description, todozi_error_t** error);

// add_checklist_item
TODOZI void todozi_add_checklist_item(const char* item);

// add_chunk
TODOZI void todozi_add_chunk(const char* chunk_id, const char* level, void* deps);

// add_completed_module
TODOZI void todozi_add_completed_module(const char* module);

// add_dependency
TODOZI void todozi_add_dependency(const char* dep);

// add_error_pattern
TODOZI void todozi_add_error_pattern(const char* pattern);

// add_function_signature
TODOZI void todozi_add_function_signature(const char* name, const char* signature);

// add_import
TODOZI void todozi_add_import(const char* import_stmt);

// add_item
TODOZI void todozi_add_item(const char* content);

// add_key
TODOZI void todozi_add_key(void* key);

// add_pending_module
TODOZI void todozi_add_pending_module(const char* module);

// add_processed_action
TODOZI const char* todozi_add_processed_action(const char* action_type, const char* description, bool success, const char* result, todozi_error_t** error);

// add_queue_item
TODOZI void todozi_add_queue_item(todozi_queue_item_t* item);

// add_recent
TODOZI void todozi_add_recent(const char* description);

// add_recent_action
TODOZI void todozi_add_recent_action(const char* action);

// add_tag_relationship
TODOZI void todozi_add_tag_relationship(const char* tag_id, const char* related_tag_id);

// add_tag_to_task
TODOZI void todozi_add_tag_to_task(const char* task_id, const char* tag);

// add_task
TODOZI const char* todozi_add_task(todozi_task_t* task, todozi_error_t** error);

// add_task_emb
TODOZI const char* todozi_add_task_emb(void* task, todozi_error_t** error);

// add_task_to_project
TODOZI void todozi_add_task_to_project(todozi_task_t* task);

// add_to_task
TODOZI void todozi_add_to_task(const char* task_id, const char* tag);

// advanced_search
TODOZI void* todozi_advanced_search(const char* query, todozi_error_t** error);

// advanced_search_with_filters
TODOZI void* todozi_advanced_search_with_filters(const char* query, void* data_types, uint32_t limit, todozi_error_t** error);

// ai
TODOZI const char* todozi_ai(const char* action, todozi_error_t** error);

// ai_find
TODOZI void* todozi_ai_find(const char* query, todozi_error_t** error);

// ai_search
TODOZI void* todozi_ai_search(const char* query, todozi_error_t** error);

// ai_task
TODOZI const char* todozi_ai_task(const char* action, todozi_error_t** error);

// ai_tasks
TODOZI void* todozi_ai_tasks(const char* query, todozi_error_t** error);

// all
TODOZI void* todozi_all(todozi_error_t** error);

// all_tasks
TODOZI void* todozi_all_tasks(todozi_error_t** error);

// analyze_code_quality
TODOZI void* todozi_analyze_code_quality(const char* _features, todozi_error_t** error);

// api
TODOZI void* todozi_api_call(void* message, todozi_error_t** error);

// api_key
TODOZI const char* TODOZI_key(todozi_error_t** error);

// archive_project
TODOZI void todozi_archive_project(const char* name);

// as_str
TODOZI const char* todozi_as_str(todozi_error_t** error);

// assign_task_to_agent
TODOZI const char* todozi_assign_task_to_agent(const char* task_id, const char* agent_id, const char* project_id, todozi_error_t** error);

// auto_categorize_content
TODOZI const char* todozi_auto_categorize_content(const char* text, todozi_error_t** error);

// auto_label_clusters
TODOZI void* todozi_auto_label_clusters(void* clusters, todozi_error_t** error);

// backlog
TODOZI void* todozi_backlog(todozi_error_t** error);

// backup_embeddings
TODOZI const char* todozi_backup_embeddings(const char* backup_path, todozi_error_t** error);

// batch_embed_content
TODOZI void* todozi_batch_embed_content(void* content_list, todozi_error_t** error);

// begin
TODOZI void todozi_begin(const char* task_id);

// breakthrough
TODOZI const char* todozi_breakthrough(const char* idea, todozi_error_t** error);

// breakthrough_idea
TODOZI const char* todozi_breakthrough_idea(const char* idea, todozi_error_t** error);

// breakthrough_percentage
TODOZI double todozi_breakthrough_percentage(todozi_error_t** error);

// build_similarity_graph
TODOZI void* todozi_build_similarity_graph(float threshold, todozi_error_t** error);

// bulk_create_tags
TODOZI void* todozi_bulk_create_tags(void* tag_names, const char* category, todozi_error_t** error);

// calculate_diversity
TODOZI float todozi_calculate_diversity(void* content_ids, todozi_error_t** error);

// capabilities
TODOZI void* todozi_capabilities(void* capabilities, todozi_error_t** error);

// category
TODOZI void* todozi_category(const char* category, todozi_error_t** error);

// chat
TODOZI void* todozi_chat(const char* message, todozi_error_t** error);

// check_api_key_auth
TODOZI void* todozi_check_api_key_auth(const char* public_key, const char* private_key, todozi_error_t** error);

// check_folder_structure
TODOZI bool todozi_check_folder_structure(todozi_error_t** error);

// check_migration_status
TODOZI const char* todozi_check_migration_status(todozi_error_t** error);

// cleanup_expired
TODOZI size_t todozi_cleanup_expired(todozi_error_t** error);

// cleanup_legacy
TODOZI void todozi_cleanup_legacy(void);

// clear_registration
TODOZI void todozi_clear_registration(void);

// cli_add_task
TODOZI void todozi_cli_add_task(const char* content, const char* priority);

// cli_chat
TODOZI const char* todozi_cli_chat(const char* message, todozi_error_t** error);

// cli_clear_registration
TODOZI void todozi_cli_clear_registration(void);

// cli_complete_task
TODOZI void todozi_cli_complete_task(const char* task_id);

// cli_create_backup
TODOZI const char* todozi_cli_create_backup(todozi_error_t** error);

// cli_create_idea
TODOZI void todozi_cli_create_idea(const char* content);

// cli_create_memory
TODOZI void todozi_cli_create_memory(const char* moment, const char* meaning, const char* reason);

// cli_delete_task
TODOZI void todozi_cli_delete_task(const char* task_id);

// cli_fix_consistency
TODOZI void todozi_cli_fix_consistency(void);

// cli_get_registration_status
TODOZI void* todozi_cli_get_registration_status(todozi_error_t** error);

// cli_list_backups
TODOZI void* todozi_cli_list_backups(todozi_error_t** error);

// cli_list_tasks
TODOZI void* todozi_cli_list_tasks(todozi_error_t** error);

// cli_register_with_server
TODOZI void todozi_cli_register_with_server(const char* server_url);

// cli_restore_backup
TODOZI void todozi_cli_restore_backup(const char* backup_name);

// cli_search_tasks
TODOZI void* todozi_cli_search_tasks(const char* query, todozi_error_t** error);

// cli_show_task
TODOZI void* todozi_cli_show_task(const char* task_id, todozi_error_t** error);

// cli_update_task
TODOZI void todozi_cli_update_task(const char* task_id, const char* action, const char* priority, const char* status);

// cluster
TODOZI void* todozi_cluster(todozi_error_t** error);

// cluster_content
TODOZI void* todozi_cluster_content(todozi_error_t** error);

// cluster_similar_tasks
TODOZI void* todozi_cluster_similar_tasks(todozi_error_t** error);

// collab
TODOZI const char* todozi_collab(const char* action, todozi_error_t** error);

// collab_task
TODOZI const char* todozi_collab_task(const char* action, todozi_error_t** error);

// color
TODOZI void* todozi_color(const char* color, todozi_error_t** error);

// compare_models
TODOZI void* todozi_compare_models(const char* text, void* model_aliases, todozi_error_t** error);

// complete
TODOZI void todozi_complete(const char* task_id);

// complete_agent_assignment
TODOZI void todozi_complete_agent_assignment(const char* task_id);

// complete_task
TODOZI void todozi_complete_task(const char* task_id);

// complete_task_in_project
TODOZI void todozi_complete_task_in_project(const char* id);

// completion_rate
TODOZI double todozi_completion_rate(todozi_error_t** error);

// config
TODOZI const char* todozi_config(todozi_error_t** error);

// configure_display
TODOZI const char* todozi_configure_display(const char* config_json, todozi_error_t** error);

// configure_server
TODOZI const char* todozi_configure_server(const char* host, uint32_t port, uint32_t max_connections, todozi_error_t** error);

// content
TODOZI void* todozi_content(const char* content, todozi_error_t** error);

// context
TODOZI void* todozi_context(const char* context, todozi_error_t** error);

// craft_embedding
TODOZI void* todozi_craft_embedding(const char* _features, todozi_error_t** error);

// create
TODOZI const char* todozi_create(const char* name, const char* description, todozi_error_t** error);

// create_advanced_todozi_tools
TODOZI void* todozi_create_advanced_todozi_tools(void* todozi, todozi_error_t** error);

// create_agent
TODOZI const char* todozi_create_agent(void* agent, todozi_error_t** error);

// create_api_key
TODOZI void* todozi_create_api_key(todozi_error_t** error);

// create_api_key_with_user_id
TODOZI void* todozi_create_api_key_with_user_id(const char* user_id, todozi_error_t** error);

// create_architect_agent
TODOZI void* todozi_create_architect_agent(todozi_error_t** error);

// create_backup
TODOZI const char* todozi_create_backup(todozi_error_t** error);

// create_coder
TODOZI void* todozi_create_coder(todozi_error_t** error);

// create_comrad_agent
TODOZI void* todozi_create_comrad_agent(todozi_error_t** error);

// create_custom_agent
TODOZI void* todozi_create_custom_agent(const char* id, const char* name, const char* description, void* capabilities, void* specializations, const char* category, const char* author, todozi_error_t** error);

// create_default_agents
TODOZI void todozi_create_default_agents(void);

// create_designer_agent
TODOZI void* todozi_create_designer_agent(todozi_error_t** error);

// create_detective_agent
TODOZI void* todozi_create_detective_agent(todozi_error_t** error);

// create_devops_agent
TODOZI void* todozi_create_devops_agent(todozi_error_t** error);

// create_embedding_service
TODOZI void* todozi_create_embedding_service(todozi_error_t** error);

// create_embedding_version
TODOZI const char* todozi_create_embedding_version(const char* content_id, const char* version_label, todozi_error_t** error);

// create_error
TODOZI const char* todozi_create_error(void* error_obj, todozi_error_t** error);

// create_error_result
TODOZI void* todozi_create_error_result(const char* error_msg, uint64_t execution_time_ms, void* error_type, void* metadata, void* serde_json, todozi_error_t** error);

// create_filters
TODOZI void* todozi_create_filters(todozi_error_t** error);

// create_finisher_agent
TODOZI void* todozi_create_finisher_agent(todozi_error_t** error);

// create_framer_agent
TODOZI void* todozi_create_framer_agent(todozi_error_t** error);

// create_friend_agent
TODOZI void* todozi_create_friend_agent(todozi_error_t** error);

// create_grok_level_todozi_tools
TODOZI void* todozi_create_grok_level_todozi_tools(void* todozi, todozi_error_t** error);

// create_hoarder_agent
TODOZI void* todozi_create_hoarder_agent(todozi_error_t** error);

// create_idea
TODOZI const char* todozi_create_idea(todozi_idea_t* idea, todozi_error_t** error);

// create_investigator_agent
TODOZI void* todozi_create_investigator_agent(todozi_error_t** error);

// create_mason_agent
TODOZI void* todozi_create_mason_agent(todozi_error_t** error);

// create_memory
TODOZI const char* todozi_create_memory(todozi_memory_t* memory, todozi_error_t** error);

// create_nerd_agent
TODOZI void* todozi_create_nerd_agent(todozi_error_t** error);

// create_nun_agent
TODOZI void* todozi_create_nun_agent(todozi_error_t** error);

// create_overlord_agent
TODOZI void* todozi_create_overlord_agent(todozi_error_t** error);

// create_party_agent
TODOZI void* todozi_create_party_agent(todozi_error_t** error);

// create_planner_agent
TODOZI void* todozi_create_planner_agent(todozi_error_t** error);

// create_project
TODOZI const char* todozi_create_project(const char* name, const char* description, todozi_error_t** error);

// create_recycler_agent
TODOZI void* todozi_create_recycler_agent(todozi_error_t** error);

// create_reminder
TODOZI const char* todozi_create_reminder(void* reminder, todozi_error_t** error);

// create_search_options
TODOZI const char* todozi_create_search_options(void* data_types, uint32_t limit, todozi_error_t** error);

// create_skeleton_agent
TODOZI void* todozi_create_skeleton_agent(todozi_error_t** error);

// create_snitch_agent
TODOZI void* todozi_create_snitch_agent(todozi_error_t** error);

// create_storage
TODOZI void* todozi_create_storage(todozi_error_t** error);

// create_success_result
TODOZI void* todozi_create_success_result(const char* output, uint64_t execution_time_ms, void* metadata, void* serde_json, todozi_error_t** error);

// create_summary
TODOZI const char* todozi_create_summary(void* summary, todozi_error_t** error);

// create_tag
TODOZI const char* todozi_create_tag(void* tag, todozi_error_t** error);

// create_task
TODOZI todozi_task_t* todozi_create_task(const char* action, void* priority, const char* project, const char* time, const char* context, todozi_error_t** error);

// create_task_filters
TODOZI void* todozi_create_task_filters(const char* project, const char* status, const char* priority, const char* assignee, const char* tags, const char* search, todozi_error_t** error);

// create_tdz_content_processor_tool
TODOZI void* todozi_create_tdz_content_processor_tool(void* state, todozi_error_t** error);

// create_tdz_tool
TODOZI const char* todozi_create_tdz_tool(todozi_error_t** error);

// create_tester_agent
TODOZI void* todozi_create_tester_agent(todozi_error_t** error);

// create_todozi_tools
TODOZI void* todozi_create_todozi_tools(void* todozi, todozi_error_t** error);

// create_todozi_tools_with_embedding
TODOZI void* todozi_create_todozi_tools_with_embedding(void* todozi, void* embedding_service, todozi_error_t** error);

// create_tool_definition
TODOZI void* todozi_create_tool_definition(const char* name, const char* description, const char* category, void* parameters, todozi_error_t** error);

// create_tool_definition_with_locks
TODOZI void* todozi_create_tool_definition_with_locks(const char* name, const char* description, const char* category, void* parameters, void* resource_locks, todozi_error_t** error);

// create_tool_parameter
TODOZI void* todozi_create_tool_parameter(const char* name, const char* type_, const char* description, bool required, todozi_error_t** error);

// create_tool_parameter_with_default
TODOZI void* todozi_create_tool_parameter_with_default(const char* name, const char* type_, const char* description, bool required, const char* default_value, todozi_error_t** error);

// create_tui_app
TODOZI const char* todozi_create_tui_app(todozi_error_t** error);

// create_tuner_agent
TODOZI void* todozi_create_tuner_agent(todozi_error_t** error);

// create_update
TODOZI void* todozi_create_update(todozi_error_t** error);

// create_writer_agent
TODOZI void* todozi_create_writer_agent(todozi_error_t** error);

// critical_percentage
TODOZI double todozi_critical_percentage(todozi_error_t** error);

// deactivate_api_key
TODOZI void todozi_deactivate_api_key(const char* user_id);

// deactivate_key
TODOZI bool todozi_deactivate_key(const char* user_id, todozi_error_t** error);

// deep
TODOZI void* todozi_deep(const char* query, todozi_error_t** error);

// deep_search
TODOZI void* todozi_deep_search(const char* query, todozi_error_t** error);

// default_filters
TODOZI void* todozi_default_filters(todozi_error_t** error);

// default_update
TODOZI void* todozi_default_update(todozi_error_t** error);

// delete
TODOZI void todozi_delete(const char* task_id);

// delete_agent
TODOZI void todozi_delete_agent(const char* agent_id);

// delete_agent_assignment
TODOZI void todozi_delete_agent_assignment(const char* agent_id, const char* task_id);

// delete_code_chunk
TODOZI void todozi_delete_code_chunk(const char* chunk_id);

// delete_error
TODOZI void todozi_delete_error(const char* error_id);

// delete_feeling
TODOZI void todozi_delete_feeling(const char* id);

// delete_idea
TODOZI void todozi_delete_idea(const char* idea_id);

// delete_memory
TODOZI void todozi_delete_memory(const char* memory_id);

// delete_project
TODOZI void todozi_delete_project(const char* project_name);

// delete_project_task_container
TODOZI void todozi_delete_project_task_container(const char* project_name);

// delete_reminder
TODOZI void todozi_delete_reminder(const char* reminder_id);

// delete_summary
TODOZI void todozi_delete_summary(const char* summary_id);

// delete_tag
TODOZI void todozi_delete_tag(const char* tag_id);

// delete_task
TODOZI void todozi_delete_task(const char* task_id);

// delete_task_from_project
TODOZI void todozi_delete_task_from_project(const char* id);

// delete_training_data
TODOZI void todozi_delete_training_data(const char* training_data_id);

// description
TODOZI void* todozi_description(const char* description, todozi_error_t** error);

// detailed
TODOZI const char* todozi_detailed(todozi_error_t** error);

// detailed_stats
TODOZI const char* todozi_detailed_stats(todozi_error_t** error);

// display_task
TODOZI void* todozi_display_task(const char* task_id, todozi_error_t** error);

// display_tasks
TODOZI void* todozi_display_tasks(void* task_ids, todozi_error_t** error);

// do_it
TODOZI const char* todozi_do_it(const char* what, todozi_error_t** error);

// done
TODOZI void todozi_done(const char* task_id);

// done_api_key
TODOZI const char* todozi_done_api_key(todozi_error_t** error);

// done_create_embedding_service
TODOZI const char* todozi_done_create_embedding_service(todozi_error_t** error);

// done_create_storage
TODOZI const char* todozi_done_create_storage(todozi_error_t** error);

// done_default_filters
TODOZI const char* todozi_done_default_filters(todozi_error_t** error);

// done_default_update
TODOZI const char* todozi_done_default_update(todozi_error_t** error);

// done_embedding_config
TODOZI const char* todozi_done_embedding_config(todozi_error_t** error);

// done_embedding_service
TODOZI const char* todozi_done_embedding_service(todozi_error_t** error);

// done_init
TODOZI void todozi_done_init(void);

// done_process_chat
TODOZI const char* todozi_done_process_chat(const char* message, const char* user_id, todozi_error_t** error);

// done_sample_task
TODOZI void* todozi_done_sample_task(todozi_error_t** error);

// done_storage
TODOZI const char* todozi_done_storage(todozi_error_t** error);

// done_types
TODOZI const char* todozi_done_types(todozi_error_t** error);

// dry_run
TODOZI void* todozi_dry_run(bool dry_run, todozi_error_t** error);

// easy_done
TODOZI void todozi_easy_done(const char* task_id);

// easy_find
TODOZI const char* todozi_easy_find(const char* what, todozi_error_t** error);

// easy_idea
TODOZI const char* todozi_easy_idea(const char* what, todozi_error_t** error);

// easy_remember
TODOZI const char* todozi_easy_remember(const char* what, todozi_error_t** error);

// embed
TODOZI void* todozi_embed(const char* text, todozi_error_t** error);

// embed_idea
TODOZI const char* todozi_embed_idea(const char* idea, todozi_error_t** error);

// embed_memory
TODOZI const char* todozi_embed_memory(const char* memory, todozi_error_t** error);

// embed_stats
TODOZI const char* todozi_embed_stats(todozi_error_t** error);

// embed_tag
TODOZI const char* todozi_embed_tag(const char* tag, todozi_error_t** error);

// embed_task
TODOZI const char* todozi_embed_task(const char* task_id, todozi_error_t** error);

// embedding_config
TODOZI void* todozi_embedding_config(todozi_error_t** error);

// embedding_service
TODOZI void* todozi_embedding_service(todozi_error_t** error);

// encode
TODOZI void* todozi_encode(const char* texts, todozi_error_t** error);

// end_queue_session
TODOZI void todozi_end_queue_session(const char* session_id);

// end_session
TODOZI void todozi_end_session(const char* session_id);

// ensure_folder_structure
TODOZI bool todozi_ensure_folder_structure(todozi_error_t** error);

// ensure_todozi_initialized
TODOZI void todozi_ensure_todozi_initialized(void);

// error
TODOZI void* todozi_error(const char* error_msg, uint64_t execution_time_ms, todozi_error_t** error);

// example
TODOZI const char* todozi_example(todozi_error_t** error);

// example_usage
TODOZI void* todozi_example_usage(todozi_error_t** error);

// execute_task
TODOZI const char* todozi_execute_task(const char* storage, const char* task, todozi_error_t** error);

// execute_tdz_command
TODOZI void* todozi_execute_tdz_command(const char* command, const char* base_url, const char* api_key, todozi_error_t** error);

// execute_todozi_tool_delegated
TODOZI void* todozi_execute_todozi_tool_delegated(const char* params, todozi_error_t** error);

// execute_tool
TODOZI void* todozi_execute_tool(const char* tool_name, void* kwargs, void* serde_json, todozi_error_t** error);

// explain_search_result
TODOZI const char* todozi_explain_search_result(const char* query, const char* result, todozi_error_t** error);

// export_diagnostics
TODOZI void* todozi_export_diagnostics(todozi_error_t** error);

// export_embedded_tasks_hlx
TODOZI void todozi_export_embedded_tasks_hlx(const char* output_path);

// export_for_fine_tuning
TODOZI size_t todozi_export_for_fine_tuning(const char* output_path, todozi_error_t** error);

// export_to_format
TODOZI const char* todozi_export_to_format(const char* format, todozi_error_t** error);

// extract_content
TODOZI const char* todozi_extract_content(const char* content, const char* file_path, const char* output_format, bool human, todozi_error_t** error);

// extract_content_from_text
TODOZI const char* todozi_extract_content_from_text(const char* text, const char* format, todozi_error_t** error);

// extract_task_actions
TODOZI void* todozi_extract_task_actions(const char* content, todozi_error_t** error);

// extract_tasks
TODOZI void* todozi_extract_tasks(const char* content, const char* context, todozi_error_t** error);

// fast
TODOZI const char* todozi_fast(const char* query, todozi_error_t** error);

// fast_search
TODOZI const char* todozi_fast_search(const char* query, todozi_error_t** error);

// filtered_semantic_search
TODOZI void* todozi_filtered_semantic_search(const char* query, void* filters, size_t limit, todozi_error_t** error);

// find
TODOZI void* todozi_find(const char* query, todozi_error_t** error);

// find_best_agent
TODOZI const char* todozi_find_best_agent(const char* required_specialization, const char* preferred_capability, todozi_error_t** error);

// find_by_tag
TODOZI void* todozi_find_by_tag(const char* tag_name, todozi_error_t** error);

// find_cross_content_relationships
TODOZI void* todozi_find_cross_content_relationships(const char* content_id, void* content_type, float min_similarity, todozi_error_t** error);

// find_ideas
TODOZI void* todozi_find_ideas(const char* query, todozi_error_t** error);

// find_memories
TODOZI void* todozi_find_memories(const char* query, todozi_error_t** error);

// find_outliers
TODOZI void* todozi_find_outliers(void* content_type, float threshold, todozi_error_t** error);

// find_similar_tags
TODOZI void* todozi_find_similar_tags(const char* tag_name, size_t limit, todozi_error_t** error);

// find_similar_tasks
TODOZI void* todozi_find_similar_tasks(const char* task_description, size_t limit, todozi_error_t** error);

// find_tasks
TODOZI void* todozi_find_tasks(const char* query, todozi_error_t** error);

// find_tasks_ai
TODOZI void* todozi_find_tasks_ai(const char* query, todozi_error_t** error);

// find_tdz
TODOZI const char* todozi_find_tdz(const char* str, todozi_error_t** error);

// find_todozi
TODOZI const char* todozi_find_todozi(const char* str, todozi_error_t** error);

// fix_completed_tasks_consistency
TODOZI void todozi_fix_completed_tasks_consistency(void);

// fix_task_consistency
TODOZI void todozi_fix_task_consistency(void);

// force_overwrite
TODOZI void* todozi_force_overwrite(bool force_overwrite, todozi_error_t** error);

// format_duration
TODOZI const char* todozi_format_duration(void* from, void* to, todozi_error_t** error);

// format_error_with_context
TODOZI const char* todozi_format_error_with_context(const char* error_message, const char* context, todozi_error_t** error);

// format_project_stats
TODOZI const char* todozi_format_project_stats(const char* project_name, size_t task_count, size_t completed_count, todozi_error_t** error);

// format_task
TODOZI const char* todozi_format_task(const char* task, todozi_error_t** error);

// format_task_list
TODOZI const char* todozi_format_task_list(const char* tasks, todozi_error_t** error);

// format_task_with_emojis
TODOZI const char* todozi_format_task_with_emojis(const char* task, todozi_error_t** error);

// format_time_estimate
TODOZI const char* todozi_format_time_estimate(const char* time, todozi_error_t** error);

// fuzzy_search
TODOZI void* todozi_fuzzy_search(const char* query, size_t max_distance, todozi_error_t** error);

// generate_embedding
TODOZI void* todozi_generate_embedding(const char* text, todozi_error_t** error);

// generate_embeddings_batch
TODOZI void* todozi_generate_embeddings_batch(void* texts, todozi_error_t** error);

// generate_task_embedding
TODOZI void* todozi_generate_task_embedding(const char* task, todozi_error_t** error);

// get
TODOZI todozi_task_t* todozi_get(const char* task_id, todozi_error_t** error);

// get_active_items
TODOZI void* todozi_get_active_items(todozi_error_t** error);

// get_active_keys
TODOZI void* todozi_get_active_keys(todozi_error_t** error);

// get_active_reminders
TODOZI void* todozi_get_active_reminders(todozi_error_t** error);

// get_active_sessions
TODOZI void* todozi_get_active_sessions(todozi_error_t** error);

// get_agent
TODOZI const char* todozi_get_agent(const char* agent_id, todozi_error_t** error);

// get_agent_assignments
TODOZI void* todozi_get_agent_assignments(const char* agent_id, todozi_error_t** error);

// get_agent_assignments_dir
TODOZI void* todozi_get_agent_assignments_dir(const char* agent_id, todozi_error_t** error);

// get_agent_statistics
TODOZI const char* todozi_get_agent_statistics(todozi_error_t** error);

// get_agents_by_capability
TODOZI void* todozi_get_agents_by_capability(const char* capability, todozi_error_t** error);

// get_agents_by_specialization
TODOZI void* todozi_get_agents_by_specialization(const char* specialization, todozi_error_t** error);

// get_agents_dir
TODOZI void* todozi_get_agents_dir(todozi_error_t** error);

// get_agents_with_assignments
TODOZI void* todozi_get_agents_with_assignments(todozi_error_t** error);

// get_ai_tasks
TODOZI void* todozi_get_ai_tasks(todozi_error_t** error);

// get_all_active_tasks
TODOZI void* todozi_get_all_active_tasks(todozi_error_t** error);

// get_all_agents
TODOZI void* todozi_get_all_agents(todozi_error_t** error);

// get_all_categories
TODOZI void* todozi_get_all_categories(todozi_error_t** error);

// get_all_completed_tasks
TODOZI void* todozi_get_all_completed_tasks(todozi_error_t** error);

// get_all_ideas
TODOZI void* todozi_get_all_ideas(todozi_error_t** error);

// get_all_items
TODOZI void* todozi_get_all_items(todozi_error_t** error);

// get_all_keys
TODOZI void* todozi_get_all_keys(todozi_error_t** error);

// get_all_memories
TODOZI void* todozi_get_all_memories(todozi_error_t** error);

// get_all_reminders
TODOZI void* todozi_get_all_reminders(todozi_error_t** error);

// get_all_summaries
TODOZI void* todozi_get_all_summaries(todozi_error_t** error);

// get_all_tags
TODOZI void* todozi_get_all_tags(todozi_error_t** error);

// get_all_tasks
TODOZI void* todozi_get_all_tasks(todozi_error_t** error);

// get_all_tools
TODOZI void* todozi_get_all_tools(todozi_error_t** error);

// get_api_key
TODOZI void* todozi_get_api_key(const char* user_id, todozi_error_t** error);

// get_api_key_by_public
TODOZI void* todozi_get_api_key_by_public(const char* public_key, todozi_error_t** error);

// get_assignee_emoji
TODOZI const char* todozi_get_assignee_emoji(const char* assignee, todozi_error_t** error);

// get_assignments_dir
TODOZI void* todozi_get_assignments_dir(todozi_error_t** error);

// get_available_agents
TODOZI void* todozi_get_available_agents(todozi_error_t** error);

// get_backlog_items
TODOZI void* todozi_get_backlog_items(todozi_error_t** error);

// get_breakthrough_ideas
TODOZI void* todozi_get_breakthrough_ideas(todozi_error_t** error);

// get_chunk
TODOZI const char* todozi_get_chunk(const char* chunk_id, todozi_error_t** error);

// get_chunk_mut
TODOZI char* todozi_get_chunk_mut(const char* chunk_id, todozi_error_t** error);

// get_chunk_status
TODOZI const char* todozi_get_chunk_status(const char* chunk_id, todozi_error_t** error);

// get_chunking_level_info
TODOZI const char* todozi_get_chunking_level_info(const char* level, todozi_error_t** error);

// get_chunking_levels
TODOZI void* todozi_get_chunking_levels(todozi_error_t** error);

// get_chunks_by_level
TODOZI void* todozi_get_chunks_by_level(void* level, todozi_error_t** error);

// get_chunks_dir
TODOZI void* todozi_get_chunks_dir(todozi_error_t** error);

// get_collaborative_tasks
TODOZI void* todozi_get_collaborative_tasks(todozi_error_t** error);

// get_command_documentation
TODOZI const char* todozi_get_command_documentation(const char* command_name, todozi_error_t** error);

// get_complete_items
TODOZI void* todozi_get_complete_items(todozi_error_t** error);

// get_critical_memories
TODOZI void* todozi_get_critical_memories(todozi_error_t** error);

// get_current_duration
TODOZI uint64_t todozi_get_current_duration(todozi_error_t** error);

// get_default_model
TODOZI const char* todozi_get_default_model(todozi_error_t** error);

// get_dependency_chain
TODOZI void* todozi_get_dependency_chain(const char* chunk_id, todozi_error_t** error);

// get_embedding_statistics
TODOZI const char* todozi_get_embedding_statistics(todozi_error_t** error);

// get_emotional_memories
TODOZI void* todozi_get_emotional_memories(const char* emotion, todozi_error_t** error);

// get_enabled_tools
TODOZI void* todozi_get_enabled_tools(todozi_error_t** error);

// get_error_details
TODOZI void* todozi_get_error_details(const char* error_message, todozi_error_t** error);

// get_errors_dir
TODOZI void* todozi_get_errors_dir(todozi_error_t** error);

// get_filtered_tasks
TODOZI void* todozi_get_filtered_tasks(const char* filters, todozi_error_t** error);

// get_formatted_prompt
TODOZI const char* todozi_get_formatted_prompt(const char* variables, todozi_error_t** error);

// get_high_priority_summaries
TODOZI void* todozi_get_high_priority_summaries(todozi_error_t** error);

// get_human_memories
TODOZI void* todozi_get_human_memories(todozi_error_t** error);

// get_human_tasks
TODOZI void* todozi_get_human_tasks(todozi_error_t** error);

// get_idea
TODOZI const char* todozi_get_idea(const char* idea_id, todozi_error_t** error);

// get_idea_statistics
TODOZI void* todozi_get_idea_statistics(todozi_error_t** error);

// get_ideas_by_importance
TODOZI void* todozi_get_ideas_by_importance(void* importance, todozi_error_t** error);

// get_ideas_by_share_level
TODOZI void* todozi_get_ideas_by_share_level(void* share_level, todozi_error_t** error);

// get_ideas_by_tag
TODOZI void* todozi_get_ideas_by_tag(const char* tag, todozi_error_t** error);

// get_ideas_dir
TODOZI void* todozi_get_ideas_dir(todozi_error_t** error);

// get_item
TODOZI const char* todozi_get_item(const char* id, todozi_error_t** error);

// get_item_mut
TODOZI char* todozi_get_item_mut(const char* id, todozi_error_t** error);

// get_items_by_status
TODOZI void* todozi_get_items_by_status(void* status, todozi_error_t** error);

// get_key
TODOZI const char* todozi_get_key(const char* user_id, todozi_error_t** error);

// get_key_by_public
TODOZI const char* todozi_get_key_by_public(const char* public_key, todozi_error_t** error);

// get_long_term_memories
TODOZI void* todozi_get_long_term_memories(todozi_error_t** error);

// get_memories_by_importance
TODOZI void* todozi_get_memories_by_importance(void* importance, todozi_error_t** error);

// get_memories_by_tag
TODOZI void* todozi_get_memories_by_tag(const char* tag, todozi_error_t** error);

// get_memories_by_term
TODOZI void* todozi_get_memories_by_term(void* term, todozi_error_t** error);

// get_memories_by_type
TODOZI void* todozi_get_memories_by_type(const char* memory_type, todozi_error_t** error);

// get_memories_dir
TODOZI void* todozi_get_memories_dir(todozi_error_t** error);

// get_memory
TODOZI const char* todozi_get_memory(const char* memory_id, todozi_error_t** error);

// get_memory_statistics
TODOZI void* todozi_get_memory_statistics(todozi_error_t** error);

// get_most_used_tags
TODOZI void* todozi_get_most_used_tags(size_t limit, todozi_error_t** error);

// get_next_chunk_to_work_on
TODOZI const char* todozi_get_next_chunk_to_work_on(todozi_error_t** error);

// get_or_generate_embedding
TODOZI void* todozi_get_or_generate_embedding(const char* content_id, const char* text, void* content_type, bool refresh_if_stale, todozi_error_t** error);

// get_overdue_reminders
TODOZI void* todozi_get_overdue_reminders(todozi_error_t** error);

// get_pending_reminders
TODOZI void* todozi_get_pending_reminders(todozi_error_t** error);

// get_priority_emoji
TODOZI const char* todozi_get_priority_emoji(const char* priority, todozi_error_t** error);

// get_private_ideas
TODOZI void* todozi_get_private_ideas(todozi_error_t** error);

// get_processor_state
TODOZI const char* todozi_get_processor_state(todozi_error_t** error);

// get_project
TODOZI todozi_project_t* todozi_get_project(const char* name, todozi_error_t** error);

// get_project_stats
TODOZI void* todozi_get_project_stats(const char* project_name, todozi_error_t** error);

// get_project_summary
TODOZI const char* todozi_get_project_summary(todozi_error_t** error);

// get_project_tasks
TODOZI void* todozi_get_project_tasks(const char* project_name, todozi_error_t** error);

// get_project_tasks_dir
TODOZI void* todozi_get_project_tasks_dir(todozi_error_t** error);

// get_public_ideas
TODOZI void* todozi_get_public_ideas(todozi_error_t** error);

// get_queue_item
TODOZI todozi_queue_item_t* todozi_get_queue_item(const char* id, todozi_error_t** error);

// get_queue_session
TODOZI void* todozi_get_queue_session(const char* session_id, todozi_error_t** error);

// get_queue_statistics
TODOZI const char* todozi_get_queue_statistics(todozi_error_t** error);

// get_ready_chunks
TODOZI void* todozi_get_ready_chunks(todozi_error_t** error);

// get_recent_actions
TODOZI void* todozi_get_recent_actions(uint32_t limit, todozi_error_t** error);

// get_recent_ideas
TODOZI void* todozi_get_recent_ideas(size_t limit, todozi_error_t** error);

// get_recent_memories
TODOZI void* todozi_get_recent_memories(size_t limit, todozi_error_t** error);

// get_recent_reminders
TODOZI void* todozi_get_recent_reminders(size_t limit, todozi_error_t** error);

// get_recent_summaries
TODOZI void* todozi_get_recent_summaries(size_t limit, todozi_error_t** error);

// get_recent_tags
TODOZI void* todozi_get_recent_tags(size_t limit, todozi_error_t** error);

// get_registration_info
TODOZI void* todozi_get_registration_info(todozi_error_t** error);

// get_related_tags
TODOZI void* todozi_get_related_tags(const char* tag_id, todozi_error_t** error);

// get_reminder
TODOZI void* todozi_get_reminder(const char* reminder_id, todozi_error_t** error);

// get_reminder_statistics
TODOZI const char* todozi_get_reminder_statistics(todozi_error_t** error);

// get_reminders_by_priority
TODOZI void* todozi_get_reminders_by_priority(void* priority, todozi_error_t** error);

// get_reminders_by_status
TODOZI void* todozi_get_reminders_by_status(void* status, todozi_error_t** error);

// get_reminders_by_tag
TODOZI void* todozi_get_reminders_by_tag(const char* tag, todozi_error_t** error);

// get_reminders_due_soon
TODOZI void* todozi_get_reminders_due_soon(void* duration, todozi_error_t** error);

// get_search_analytics
TODOZI void* todozi_get_search_analytics(todozi_error_t** error);

// get_search_history
TODOZI void* todozi_get_search_history(uint32_t limit, todozi_error_t** error);

// get_search_suggestions
TODOZI void* todozi_get_search_suggestions(const char* query, size_t limit, todozi_error_t** error);

// get_secret_memories
TODOZI void* todozi_get_secret_memories(todozi_error_t** error);

// get_server_status
TODOZI const char* todozi_get_server_status(todozi_error_t** error);

// get_session
TODOZI const char* todozi_get_session(const char* id, todozi_error_t** error);

// get_short_term_memories
TODOZI void* todozi_get_short_term_memories(todozi_error_t** error);

// get_stats
TODOZI void* todozi_get_stats(todozi_error_t** error);

// get_status_emoji
TODOZI const char* todozi_get_status_emoji(const char* status, todozi_error_t** error);

// get_storage_dir
TODOZI void* todozi_get_storage_dir(todozi_error_t** error);

// get_suggestions
TODOZI void* todozi_get_suggestions(const char* current_tags, size_t limit, todozi_error_t** error);

// get_summaries_by_priority
TODOZI void* todozi_get_summaries_by_priority(void* priority, todozi_error_t** error);

// get_summaries_by_tag
TODOZI void* todozi_get_summaries_by_tag(const char* tag, todozi_error_t** error);

// get_summary
TODOZI const char* todozi_get_summary(const char* summary_id, todozi_error_t** error);

// get_summary_statistics
TODOZI void* todozi_get_summary_statistics(todozi_error_t** error);

// get_tag
TODOZI const char* todozi_get_tag(const char* tag_id, todozi_error_t** error);

// get_tag_by_name
TODOZI const char* todozi_get_tag_by_name(const char* name, todozi_error_t** error);

// get_tag_statistics
TODOZI void* todozi_get_tag_statistics(todozi_error_t** error);

// get_tags_by_category
TODOZI void* todozi_get_tags_by_category(const char* category, todozi_error_t** error);

// get_task
TODOZI todozi_task_t* todozi_get_task(const char* id, todozi_error_t** error);

// get_task_assignments
TODOZI void* todozi_get_task_assignments(const char* task_id, todozi_error_t** error);

// get_task_from_any_project
TODOZI todozi_task_t* todozi_get_task_from_any_project(const char* id, todozi_error_t** error);

// get_task_from_project
TODOZI todozi_task_t* todozi_get_task_from_project(const char* project_name, const char* task_id, todozi_error_t** error);

// get_task_mut
TODOZI char* todozi_get_task_mut(const char* id, todozi_error_t** error);

// get_tasks_dir
TODOZI void* todozi_get_tasks_dir(todozi_error_t** error);

// get_tdz_api_key
TODOZI const char* todozi_get_tdz_api_key(todozi_error_t** error);

// get_team_ideas
TODOZI void* todozi_get_team_ideas(todozi_error_t** error);

// get_tool
TODOZI const char* todozi_get_tool(const char* name, todozi_error_t** error);

// get_tool_definitions
TODOZI void* todozi_get_tool_definitions(todozi_error_t** error);

// get_training_dir
TODOZI void* todozi_get_training_dir(todozi_error_t** error);

// get_tsne_coordinates
TODOZI void* todozi_get_tsne_coordinates(void* content_ids, size_t dimensions, todozi_error_t** error);

// get_unresolved_errors
TODOZI void* todozi_get_unresolved_errors(todozi_error_t** error);

// get_version_history
TODOZI void* todozi_get_version_history(const char* content_id, todozi_error_t** error);

// handle_add_command
TODOZI void todozi_handle_add_command(void* command);

// handle_agent_command
TODOZI void todozi_handle_agent_command(void* command);

// handle_ai_commands
TODOZI void todozi_handle_ai_commands(const char* command, void* args);

// handle_api_command
TODOZI void todozi_handle_api_command(void* command);

// handle_chat_command
TODOZI void todozi_handle_chat_command(void* command);

// handle_emb_command
TODOZI void todozi_handle_emb_command(void* command);

// handle_error_command
TODOZI void todozi_handle_error_command(void* command);

// handle_extract_command
TODOZI void todozi_handle_extract_command(const char* content, const char* file, const char* output_format, bool human);

// handle_idea_command
TODOZI void todozi_handle_idea_command(void* command);

// handle_ind_command
TODOZI void todozi_handle_ind_command(void);

// handle_list_backups_command
TODOZI void todozi_handle_list_backups_command(void);

// handle_list_command
TODOZI void todozi_handle_list_command(void* command);

// handle_memory_command
TODOZI void todozi_handle_memory_command(void* command);

// handle_project_command
TODOZI void todozi_handle_project_command(void* command);

// handle_queue_command
TODOZI void todozi_handle_queue_command(void* command);

// handle_search_all_command
TODOZI void todozi_handle_search_all_command(void* command);

// handle_search_command
TODOZI void todozi_handle_search_command(void* command);

// handle_server_command
TODOZI void todozi_handle_server_command(void* command);

// handle_show_command
TODOZI void todozi_handle_show_command(void* command);

// handle_stats_command
TODOZI void todozi_handle_stats_command(void* _command);

// handle_strategy_command
TODOZI void todozi_handle_strategy_command(const char* content, const char* file, const char* output_format, bool human);

// handle_train_command
TODOZI void todozi_handle_train_command(void* command);

// handle_update_command
TODOZI void todozi_handle_update_command(const char* id, const char* action, const char* time, const char* priority, const char* project, const char* status, const char* assignee, const char* tags, const char* dependencies, const char* context, void* progress);

// has_capability
TODOZI bool todozi_has_capability(const char* capability, todozi_error_t** error);

// has_results
TODOZI bool todozi_has_results(todozi_error_t** error);

// has_specialization
TODOZI bool todozi_has_specialization(const char* specialization, todozi_error_t** error);

// has_tool
TODOZI bool todozi_has_tool(const char* tool_name, todozi_error_t** error);

// hash_project_name
TODOZI const char* todozi_hash_project_name(const char* project_name, todozi_error_t** error);

// hierarchical_clustering
TODOZI void* todozi_hierarchical_clustering(void* content_types, size_t max_depth, todozi_error_t** error);

// high
TODOZI const char* todozi_high(const char* action, todozi_error_t** error);

// high_priority_percentage
TODOZI double todozi_high_priority_percentage(todozi_error_t** error);

// human
TODOZI const char* todozi_human(const char* action, todozi_error_t** error);

// human_task
TODOZI const char* todozi_human_task(const char* action, todozi_error_t** error);

// hybrid_search
TODOZI void* todozi_hybrid_search(const char* query, void* keywords, void* content_types, float semantic_weight, size_t // 0.0-1.0
        limit, todozi_error_t** error);

// idea
TODOZI todozi_task_t* todozi_idea(const char* idea, todozi_error_t** error);

// ideate
TODOZI todozi_task_t* todozi_ideate(const char* idea, todozi_error_t** error);

// import_from_format
TODOZI const char* todozi_import_from_format(const char* data, const char* format, todozi_error_t** error);

// importance
TODOZI void* todozi_importance(void* importance, todozi_error_t** error);

// important
TODOZI const char* todozi_important(const char* moment, const char* meaning, const char* reason, todozi_error_t** error);

// important_memory
TODOZI const char* todozi_important_memory(const char* moment, const char* meaning, const char* reason, todozi_error_t** error);

// increment_tag_usage
TODOZI void todozi_increment_tag_usage(const char* tag_name);

// init
TODOZI void todozi_init(void);

// init_storage
TODOZI void todozi_init_storage(void);

// init_with_auto_registration
TODOZI void todozi_init_with_auto_registration(void);

// initialize
TODOZI void todozi_initialize(void);

// initialize_grok_level_todozi_system
TODOZI void* todozi_initialize_grok_level_todozi_system(todozi_error_t** error);

// initialize_grok_level_todozi_system_with_embedding
TODOZI void* todozi_initialize_grok_level_todozi_system_with_embedding(bool enable_embeddings, todozi_error_t** error);

// initialize_tdz_content_processor
TODOZI void* todozi_initialize_tdz_content_processor(todozi_error_t** error);

// initialize_tdz_processor
TODOZI const char* todozi_initialize_tdz_processor(todozi_error_t** error);

// interactive_create_task
TODOZI todozi_task_t* todozi_interactive_create_task(todozi_error_t** error);

// io
TODOZI void* todozi_io(void* message, todozi_error_t** error);

// is_active
TODOZI bool todozi_is_active(todozi_error_t** error);

// is_admin
TODOZI bool todozi_is_admin(const char* public_key, const char* private_key, todozi_error_t** error);

// is_available
TODOZI bool todozi_is_available(todozi_error_t** error);

// is_backlog
TODOZI bool todozi_is_backlog(todozi_error_t** error);

// is_complete
TODOZI bool todozi_is_complete(todozi_error_t** error);

// is_completed
TODOZI bool todozi_is_completed(todozi_error_t** error);

// is_empty
TODOZI bool todozi_is_empty(todozi_error_t** error);

// is_overdue
TODOZI bool todozi_is_overdue(todozi_error_t** error);

// is_registered
TODOZI bool todozi_is_registered(todozi_error_t** error);

// keyword_search
TODOZI const char* todozi_keyword_search(const char* query, todozi_error_t** error);

// keyword_tasks
TODOZI void* todozi_keyword_tasks(const char* query, todozi_error_t** error);

// launch_gui
TODOZI void todozi_launch_gui(void);

// launch_tui
TODOZI const char* todozi_launch_tui(todozi_error_t** error);

// len
TODOZI size_t todozi_len(todozi_error_t** error);

// list
TODOZI void* todozi_list(todozi_error_t** error);

// list_active_api_keys
TODOZI void* todozi_list_active_api_keys(todozi_error_t** error);

// list_active_items
TODOZI void* todozi_list_active_items(todozi_error_t** error);

// list_agent_assignments
TODOZI void* todozi_list_agent_assignments(const char* agent_id, todozi_error_t** error);

// list_agents
TODOZI void* todozi_list_agents(todozi_error_t** error);

// list_all_agent_assignments
TODOZI void* todozi_list_all_agent_assignments(todozi_error_t** error);

// list_api_keys
TODOZI void* todozi_list_api_keys(todozi_error_t** error);

// list_available_agents
TODOZI void* todozi_list_available_agents(todozi_error_t** error);

// list_available_commands
TODOZI void* todozi_list_available_commands(todozi_error_t** error);

// list_available_tools
TODOZI void* todozi_list_available_tools(todozi_error_t** error);

// list_backlog_items
TODOZI void* todozi_list_backlog_items(todozi_error_t** error);

// list_backups
TODOZI void* todozi_list_backups(todozi_error_t** error);

// list_code_chunks
TODOZI void* todozi_list_code_chunks(todozi_error_t** error);

// list_complete_items
TODOZI void* todozi_list_complete_items(todozi_error_t** error);

// list_errors
TODOZI void* todozi_list_errors(todozi_error_t** error);

// list_feelings
TODOZI void* todozi_list_feelings(todozi_error_t** error);

// list_ideas
TODOZI void* todozi_list_ideas(todozi_error_t** error);

// list_memories
TODOZI void* todozi_list_memories(todozi_error_t** error);

// list_project_task_containers
TODOZI void* todozi_list_project_task_containers(todozi_error_t** error);

// list_projects
TODOZI void* todozi_list_projects(todozi_error_t** error);

// list_queue_items
TODOZI void* todozi_list_queue_items(todozi_error_t** error);

// list_queue_items_by_status
TODOZI void* todozi_list_queue_items_by_status(void* status, todozi_error_t** error);

// list_reminders
TODOZI void* todozi_list_reminders(todozi_error_t** error);

// list_summaries
TODOZI void* todozi_list_summaries(todozi_error_t** error);

// list_tasks
TODOZI void* todozi_list_tasks(todozi_error_t** error);

// list_tasks_across_projects
TODOZI void* todozi_list_tasks_across_projects(const char* filters, todozi_error_t** error);

// list_tasks_in_project
TODOZI void* todozi_list_tasks_in_project(const char* project_name, const char* filters, todozi_error_t** error);

// list_training_data
TODOZI void* todozi_list_training_data(todozi_error_t** error);

// load
TODOZI void* todozi_load(const char* model_name, void* device, todozi_error_t** error);

// load_additional_model
TODOZI void todozi_load_additional_model(const char* model_name, const char* model_alias);

// load_agent
TODOZI void* todozi_load_agent(const char* agent_id, todozi_error_t** error);

// load_agent_assignment
TODOZI void* todozi_load_agent_assignment(const char* agent_id, const char* task_id, todozi_error_t** error);

// load_agents
TODOZI void todozi_load_agents(void);

// load_api_key_collection
TODOZI void* todozi_load_api_key_collection(todozi_error_t** error);

// load_api_keys
TODOZI void todozi_load_api_keys(void);

// load_code_chunk
TODOZI void* todozi_load_code_chunk(const char* chunk_id, todozi_error_t** error);

// load_config
TODOZI void* todozi_load_config(todozi_error_t** error);

// load_error
TODOZI void* todozi_load_error(const char* error_id, todozi_error_t** error);

// load_extended_data
TODOZI void todozi_load_extended_data(void);

// load_feeling
TODOZI void* todozi_load_feeling(const char* id, todozi_error_t** error);

// load_idea
TODOZI todozi_idea_t* todozi_load_idea(const char* idea_id, todozi_error_t** error);

// load_memory
TODOZI todozi_memory_t* todozi_load_memory(const char* memory_id, todozi_error_t** error);

// load_project
TODOZI todozi_project_t* todozi_load_project(const char* project_name, todozi_error_t** error);

// load_project_task_container
TODOZI void* todozi_load_project_task_container(const char* project_name, todozi_error_t** error);

// load_project_task_container_by_hash
TODOZI void* todozi_load_project_task_container_by_hash(const char* project_hash, todozi_error_t** error);

// load_queue_collection
TODOZI void* todozi_load_queue_collection(todozi_error_t** error);

// load_task
TODOZI todozi_task_t* todozi_load_task(const char* task_id, todozi_error_t** error);

// load_task_collection
TODOZI void* todozi_load_task_collection(const char* collection_name, todozi_error_t** error);

// load_tasks
TODOZI void todozi_load_tasks(void);

// load_training_data
TODOZI void* todozi_load_training_data(const char* training_data_id, todozi_error_t** error);

// long_term_percentage
TODOZI double todozi_long_term_percentage(todozi_error_t** error);

// low
TODOZI const char* todozi_low(const char* action, todozi_error_t** error);

// mark_chunk_completed
TODOZI void todozi_mark_chunk_completed(const char* chunk_id);

// mark_chunk_validated
TODOZI void todozi_mark_chunk_validated(const char* chunk_id);

// mark_reminder_cancelled
TODOZI void todozi_mark_reminder_cancelled(const char* reminder_id);

// mark_reminder_completed
TODOZI void todozi_mark_reminder_completed(const char* reminder_id);

// matches
TODOZI bool todozi_matches(const char* public_key, const char* private_key, todozi_error_t** error);

// max_tokens
TODOZI size_t todozi_max_tokens(todozi_error_t** error);

// meaning
TODOZI void* todozi_meaning(const char* meaning, todozi_error_t** error);

// merge_tags
TODOZI void todozi_merge_tags(const char* primary_tag_id, void* duplicate_tag_ids);

// migrate
TODOZI void* todozi_migrate(todozi_error_t** error);

// migrate_tasks
TODOZI const char* todozi_migrate_tasks(bool dry_run, bool verbose, bool force_overwrite, todozi_error_t** error);

// migrate_to_project_based
TODOZI void* todozi_migrate_to_project_based(todozi_error_t** error);

// moment
TODOZI void* todozi_moment(const char* moment, todozi_error_t** error);

// move_task
TODOZI void todozi_move_task(const char* id, const char* from_collection, const char* to_collection);

// multi_query_search
TODOZI void* todozi_multi_query_search(void* queries, void* aggregation, void* content_types, size_t limit, todozi_error_t** error);

// name
TODOZI void* todozi_name(const char* name, todozi_error_t** error);

// new_full
TODOZI void* todozi_new_full(const char* user_id, const char* action, const char* time, void* priority, const char* parent_project, void* status, void* assignee, void* tags, void* dependencies, const char* context_notes, void* progress, todozi_error_t** error);

// new_idea
TODOZI const char* todozi_new_idea(todozi_idea_t* idea, todozi_error_t** error);

// new_memory
TODOZI const char* todozi_new_memory(todozi_memory_t* memory, todozi_error_t** error);

// new_with_hashes
TODOZI void* todozi_new_with_hashes(const char* server_url, todozi_error_t** error);

// ok
TODOZI void* todozi_ok(const char* body, todozi_error_t** error);

// overdue_percentage
TODOZI double todozi_overdue_percentage(todozi_error_t** error);

// parse_agent_assignment_format
TODOZI void* todozi_parse_agent_assignment_format(const char* agent_text, todozi_error_t** error);

// parse_chunking_format
TODOZI void* todozi_parse_chunking_format(const char* chunk_text, todozi_error_t** error);

// parse_dependencies
TODOZI void* todozi_parse_dependencies(const char* deps_str, todozi_error_t** error);

// parse_error_format
TODOZI void* todozi_parse_error_format(const char* error_text, todozi_error_t** error);

// parse_feeling_format
TODOZI void* todozi_parse_feeling_format(const char* feel_text, todozi_error_t** error);

// parse_idea_format
TODOZI todozi_idea_t* todozi_parse_idea_format(const char* idea_text, todozi_error_t** error);

// parse_memory_format
TODOZI todozi_memory_t* todozi_parse_memory_format(const char* memory_text, const char* user_id, todozi_error_t** error);

// parse_reminder_format
TODOZI void* todozi_parse_reminder_format(const char* reminder_text, todozi_error_t** error);

// parse_summary_format
TODOZI void* todozi_parse_summary_format(const char* summary_text, todozi_error_t** error);

// parse_tags
TODOZI void* todozi_parse_tags(const char* tags_str, todozi_error_t** error);

// parse_tdz_command
TODOZI void* todozi_parse_tdz_command(const char* text, todozi_error_t** error);

// parse_todozi_format
TODOZI todozi_task_t* todozi_parse_todozi_format(const char* todozi_text, todozi_error_t** error);

// parse_training_data_format
TODOZI void* todozi_parse_training_data_format(const char* train_text, todozi_error_t** error);

// pending_percentage
TODOZI double todozi_pending_percentage(todozi_error_t** error);

// plan_task_actions
TODOZI void* todozi_plan_task_actions(const char* goal, todozi_error_t** error);

// plan_tasks
TODOZI void* todozi_plan_tasks(const char* goal, const char* complexity, const char* timeline, const char* context, todozi_error_t** error);

// predict_relevance
TODOZI void* todozi_predict_relevance(const char* _features, todozi_error_t** error);

// preload_related_embeddings
TODOZI void todozi_preload_related_embeddings(const char* content_id, size_t depth);

// prepare_task_content
TODOZI const char* todozi_prepare_task_content(const char* task, todozi_error_t** error);

// prioritize_queue_item
TODOZI void todozi_prioritize_queue_item(const char* item_id, const char* priority);

// priority
TODOZI void* todozi_priority(void* priority, todozi_error_t** error);

// private_percentage
TODOZI double todozi_private_percentage(todozi_error_t** error);

// process_chat
TODOZI void* todozi_process_chat(const char* message, const char* user_id, todozi_error_t** error);

// process_chat_message
TODOZI void* todozi_process_chat_message(const char* message, todozi_error_t** error);

// process_chat_message_extended
TODOZI void* todozi_process_chat_message_extended(const char* message, const char* user_id, todozi_error_t** error);

// process_chunking_message
TODOZI void* todozi_process_chunking_message(const char* message, todozi_error_t** error);

// process_json_examples
TODOZI void* todozi_process_json_examples(const char* json_data, todozi_error_t** error);

// process_tdz_commands
TODOZI void* todozi_process_tdz_commands(const char* text, const char* base_url, const char* api_key, todozi_error_t** error);

// process_workflow
TODOZI void* todozi_process_workflow(void* tasks, todozi_error_t** error);

// profile_search_performance
TODOZI void* todozi_profile_search_performance(const char* query, size_t iterations, todozi_error_t** error);

// project_name
TODOZI const char* todozi_project_name(todozi_error_t** error);

// project_tasks
TODOZI void* todozi_project_tasks(const char* project_name, todozi_error_t** error);

// public_percentage
TODOZI double todozi_public_percentage(todozi_error_t** error);

// queue_active
TODOZI void* todozi_queue_active(todozi_error_t** error);

// queue_add
TODOZI const char* todozi_queue_add(const char* task_name, const char* description, todozi_error_t** error);

// queue_backlog
TODOZI void* todozi_queue_backlog(todozi_error_t** error);

// queue_bulk_operations
TODOZI const char* todozi_queue_bulk_operations(void* operations, todozi_error_t** error);

// queue_complete
TODOZI void todozi_queue_complete(const char* session_id);

// queue_list
TODOZI void* todozi_queue_list(todozi_error_t** error);

// queue_start
TODOZI const char* todozi_queue_start(const char* item_id, todozi_error_t** error);

// quick
TODOZI const char* todozi_quick(todozi_error_t** error);

// quick_task
TODOZI todozi_task_t* todozi_quick_task(const char* action, todozi_error_t** error);

// reason
TODOZI void* todozi_reason(const char* reason, todozi_error_t** error);

// recommend_similar
TODOZI void* todozi_recommend_similar(void* based_on, void* exclude, size_t limit, todozi_error_t** error);

// register_tool
TODOZI const char* todozi_register_tool(void* tool_def, todozi_error_t** error);

// register_with_server
TODOZI void* todozi_register_with_server(const char* server_url, todozi_error_t** error);

// relationships_per_tag
TODOZI double todozi_relationships_per_tag(todozi_error_t** error);

// remember
TODOZI todozi_task_t* todozi_remember(const char* moment, const char* meaning, todozi_error_t** error);

// remind_at
TODOZI void* todozi_remind_at(void* remind_at, todozi_error_t** error);

// remove_api_key
TODOZI void* todozi_remove_api_key(const char* user_id, todozi_error_t** error);

// remove_from_task
TODOZI void todozi_remove_from_task(const char* task_id, const char* tag);

// remove_item
TODOZI todozi_queue_item_t* todozi_remove_item(const char* id, todozi_error_t** error);

// remove_key
TODOZI TODOZI_key_t* todozi_remove_key(const char* user_id, todozi_error_t** error);

// remove_tag_from_task
TODOZI void todozi_remove_tag_from_task(const char* task_id, const char* tag);

// remove_task
TODOZI todozi_task_t* todozi_remove_task(const char* id, todozi_error_t** error);

// render
TODOZI const char* todozi_render(const char* config, todozi_error_t** error);

// render_compact
TODOZI const char* todozi_render_compact(const char* config, todozi_error_t** error);

// render_detailed
TODOZI const char* todozi_render_detailed(const char* config, todozi_error_t** error);

// reorder_queue
TODOZI const char* todozi_reorder_queue(void* item_ids, todozi_error_t** error);

// resolve_error
TODOZI void todozi_resolve_error(const char* error_id, const char* resolution);

// restore_backup
TODOZI void todozi_restore_backup(const char* backup_name);

// restore_embeddings
TODOZI size_t todozi_restore_embeddings(const char* backup_path, todozi_error_t** error);

// run
TODOZI void todozi_run(void);

// run_interactive
TODOZI const char* todozi_run_interactive(todozi_error_t** error);

// sample_task
TODOZI todozi_task_t* todozi_sample_task(todozi_error_t** error);

// save_agent
TODOZI void todozi_save_agent(const char* agent);

// save_agent_assignment
TODOZI void todozi_save_agent_assignment(const char* assignment);

// save_api_key_collection
TODOZI void todozi_save_api_key_collection(const char* collection);

// save_as_default
TODOZI void todozi_save_as_default(const char* model_name);

// save_code_chunk
TODOZI void todozi_save_code_chunk(const char* chunk);

// save_config
TODOZI void todozi_save_config(const char* config);

// save_error
TODOZI void todozi_save_error(const char* error);

// save_feeling
TODOZI void todozi_save_feeling(const char* feeling);

// save_idea
TODOZI void todozi_save_idea(const char* idea);

// save_memory
TODOZI void todozi_save_memory(const char* memory);

// save_processed_content
TODOZI void todozi_save_processed_content(const char* raw, const char* cleaned, const char* session_id);

// save_project
TODOZI void todozi_save_project(const char* project);

// save_project_task_container
TODOZI void todozi_save_project_task_container(const char* container);

// save_queue_collection
TODOZI void todozi_save_queue_collection(const char* collection);

// save_search_query
TODOZI const char* todozi_save_search_query(const char* query, const char* name, todozi_error_t** error);

// save_task
TODOZI void todozi_save_task(const char* task);

// save_task_collection
TODOZI void todozi_save_task_collection(const char* collection_name, const char* collection);

// save_task_with_embedding
TODOZI void todozi_save_task_with_embedding(const char* task);

// save_tasks
TODOZI void todozi_save_tasks(void);

// save_training_data
TODOZI void todozi_save_training_data(const char* training_data);

// search
TODOZI void* todozi_search(const char* query, void* options, todozi_error_t** error);

// search_ideas
TODOZI void* todozi_search_ideas(const char* query, todozi_error_t** error);

// search_memories
TODOZI void* todozi_search_memories(const char* query, todozi_error_t** error);

// search_reminders
TODOZI void* todozi_search_reminders(const char* query, todozi_error_t** error);

// search_summaries
TODOZI void* todozi_search_summaries(const char* query, todozi_error_t** error);

// search_tags
TODOZI void* todozi_search_tags(const char* query, todozi_error_t** error);

// search_tasks
TODOZI void* todozi_search_tasks(const char* query, bool semantic, size_t limit, todozi_error_t** error);

// search_tasks_semantic
TODOZI void* todozi_search_tasks_semantic(const char* query, size_t max_results, todozi_error_t** error);

// search_with_filters
TODOZI void* todozi_search_with_filters(void* filters, size_t limit, todozi_error_t** error);

// see_all
TODOZI const char* todozi_see_all(todozi_error_t** error);

// semantic_search
TODOZI void* todozi_semantic_search(const char* query, void* content_types, size_t limit, todozi_error_t** error);

// serialization
TODOZI void* todozi_serialization(void* message, todozi_error_t** error);

// set_color_scheme
TODOZI const char* todozi_set_color_scheme(const char* scheme_json, todozi_error_t** error);

// share
TODOZI void* todozi_share(void* share, todozi_error_t** error);

// short_term_percentage
TODOZI double todozi_short_term_percentage(todozi_error_t** error);

// show_loading_screen
TODOZI void todozi_show_loading_screen(void);

// similar
TODOZI void* todozi_similar(const char* query, todozi_error_t** error);

// similar_tasks
TODOZI void* todozi_similar_tasks(const char* task_id, todozi_error_t** error);

// similar_tasks_emb
TODOZI void* todozi_similar_tasks_emb(const char* query, todozi_error_t** error);

// similarity_threshold_search
TODOZI void* todozi_similarity_threshold_search(const char* query, double threshold, uint32_t limit, todozi_error_t** error);

// smart
TODOZI const char* todozi_smart(const char* query, todozi_error_t** error);

// smart_search
TODOZI const char* todozi_smart_search(const char* query, todozi_error_t** error);

// specializations
TODOZI void* todozi_specializations(void* specializations, todozi_error_t** error);

// start
TODOZI void todozi_start(const char* task_id);

// start_edit
TODOZI void todozi_start_edit(const char* _task_id);

// start_edit_session
TODOZI void* todozi_start_edit_session(const char* task_id, todozi_error_t** error);

// start_local_server
TODOZI const char* todozi_start_local_server(const char* host, uint32_t port, todozi_error_t** error);

// start_queue_session
TODOZI const char* todozi_start_queue_session(const char* queue_item_id, todozi_error_t** error);

// start_server
TODOZI void* todozi_start_server(const char* host, void* port, todozi_error_t** error);

// start_session
TODOZI const char* todozi_start_session(const char* queue_item_id, todozi_error_t** error);

// start_task
TODOZI void todozi_start_task(const char* task_id);

// stats
TODOZI const char* todozi_stats(todozi_error_t** error);

// status
TODOZI void* todozi_status(void* status, todozi_error_t** error);

// stop_server
TODOZI const char* todozi_stop_server(todozi_error_t** error);

// storage
TODOZI void* todozi_storage(todozi_error_t** error);

// storage_check_folder_structure
TODOZI bool todozi_storage_check_folder_structure(todozi_error_t** error);

// storage_clear_registration
TODOZI const char* todozi_storage_clear_registration(todozi_error_t** error);

// storage_create_backup
TODOZI const char* todozi_storage_create_backup(todozi_error_t** error);

// storage_delete_project_by_name
TODOZI void todozi_storage_delete_project_by_name(const char* name);

// storage_delete_task_from_project
TODOZI void todozi_storage_delete_task_from_project(const char* task_id);

// storage_ensure_folder_structure
TODOZI bool todozi_storage_ensure_folder_structure(todozi_error_t** error);

// storage_get_ai_tasks
TODOZI void* todozi_storage_get_ai_tasks(todozi_error_t** error);

// storage_get_all_active_tasks
TODOZI void* todozi_storage_get_all_active_tasks(todozi_error_t** error);

// storage_get_all_completed_tasks
TODOZI void* todozi_storage_get_all_completed_tasks(todozi_error_t** error);

// storage_get_collaborative_tasks
TODOZI void* todozi_storage_get_collaborative_tasks(todozi_error_t** error);

// storage_get_human_tasks
TODOZI void* todozi_storage_get_human_tasks(todozi_error_t** error);

// storage_get_project_stats
TODOZI const char* todozi_storage_get_project_stats(const char* project_name, todozi_error_t** error);

// storage_get_registration_info
TODOZI const char* todozi_storage_get_registration_info(todozi_error_t** error);

// storage_get_storage_dir
TODOZI const char* todozi_storage_get_storage_dir(todozi_error_t** error);

// storage_get_task_from_any_project
TODOZI void* todozi_storage_get_task_from_any_project(const char* task_id, todozi_error_t** error);

// storage_init
TODOZI void todozi_storage_init(void);

// storage_is_registered
TODOZI bool todozi_storage_is_registered(todozi_error_t** error);

// storage_list_backups
TODOZI void* todozi_storage_list_backups(todozi_error_t** error);

// storage_list_projects
TODOZI void* todozi_storage_list_projects(todozi_error_t** error);

// storage_load_config
TODOZI const char* todozi_storage_load_config(todozi_error_t** error);

// storage_load_project
TODOZI const char* todozi_storage_load_project(const char* name, todozi_error_t** error);

// storage_load_task_collection
TODOZI const char* todozi_storage_load_task_collection(const char* name, todozi_error_t** error);

// storage_register_with_server
TODOZI const char* todozi_storage_register_with_server(const char* server_url, todozi_error_t** error);

// storage_restore_backup
TODOZI void todozi_storage_restore_backup(const char* backup_name);

// storage_save_config
TODOZI void todozi_storage_save_config(void);

// storage_save_project
TODOZI void todozi_storage_save_project(const char* name);

// storage_save_task_collection
TODOZI void todozi_storage_save_task_collection(const char* name);

// storage_search_tasks_semantic
TODOZI void* todozi_storage_search_tasks_semantic(const char* query, uint32_t max_results, todozi_error_t** error);

// strategy_content
TODOZI const char* todozi_strategy_content(const char* content, const char* file_path, const char* output_format, bool human, todozi_error_t** error);

// strategy_content_analysis
TODOZI const char* todozi_strategy_content_analysis(const char* text, const char* format, todozi_error_t** error);

// strike_cluster
TODOZI void* todozi_strike_cluster(const char* _embedding, todozi_error_t** error);

// strike_tags
TODOZI void* todozi_strike_tags(const char* _features, todozi_error_t** error);

// success
TODOZI void* todozi_success(const char* output, uint64_t execution_time_ms, todozi_error_t** error);

// suggest_tags
TODOZI void* todozi_suggest_tags(const char* content_id, size_t top_k, todozi_error_t** error);

// tags
TODOZI void* todozi_tags(void* tags, todozi_error_t** error);

// tags_advanced_search
TODOZI void* todozi_tags_advanced_search(const char* query, todozi_error_t** error);

// task
TODOZI const char* todozi_task(const char* action, todozi_error_t** error);

// tasks
TODOZI void* todozi_tasks(const char* project_name, todozi_error_t** error);

// tdz_cnt
TODOZI const char* todozi_tdz_cnt(const char* content, const char* session_id, todozi_error_t** error);

// tdz_execute_command
TODOZI const char* todozi_tdz_execute_command(const char* command, todozi_error_t** error);

// tdz_find
TODOZI const char* todozi_tdz_find(const char* query, todozi_error_t** error);

// tdz_parse_command
TODOZI const char* todozi_tdz_parse_command(const char* input, todozi_error_t** error);

// tdzfp
TODOZI bool todozi_tdzfp(todozi_error_t** error);

// team_percentage
TODOZI double todozi_team_percentage(todozi_error_t** error);

// term
TODOZI void* todozi_term(void* term, todozi_error_t** error);

// title
TODOZI const char* todozi_title(todozi_error_t** error);

// to_context_string
TODOZI const char* todozi_to_context_string(todozi_error_t** error);

// to_ollama_format
TODOZI void* todozi_to_ollama_format(todozi_error_t** error);

// to_state_string
TODOZI const char* todozi_to_state_string(todozi_error_t** error);

// todozi_begin
TODOZI void todozi_todozi_begin(void);

// todozi_init
TODOZI void todozi_todozi_init(void);

// todozi_init_with_auto_registration
TODOZI void todozi_todozi_init_with_auto_registration(void);

// tool_count
TODOZI size_t todozi_tool_count(todozi_error_t** error);

// total_results
TODOZI size_t todozi_total_results(todozi_error_t** error);

// track_embedding_drift
TODOZI void* todozi_track_embedding_drift(const char* content_id, const char* current_text, todozi_error_t** error);

// transform_shorthand_tags
TODOZI const char* todozi_transform_shorthand_tags(const char* message, todozi_error_t** error);

// types
TODOZI const char* todozi_types(todozi_error_t** error);

// unregister
TODOZI bool todozi_unregister(const char* name, todozi_error_t** error);

// update
TODOZI void todozi_update(void* updates);

// update_agent
TODOZI void todozi_update_agent(const char* agent_id, void* updates);

// update_agent_assignment_status
TODOZI void todozi_update_agent_assignment_status(const char* agent_id, const char* task_id, void* status);

// update_agent_status
TODOZI void todozi_update_agent_status(const char* agent_id, void* status);

// update_chunk_code
TODOZI void todozi_update_chunk_code(const char* chunk_id, const char* code);

// update_chunk_tests
TODOZI void todozi_update_chunk_tests(const char* chunk_id, const char* tests);

// update_config
TODOZI void todozi_update_config(void* config);

// update_config_with_registration
TODOZI void todozi_update_config_with_registration(void* registration);

// update_feeling
TODOZI void todozi_update_feeling(const char* feeling);

// update_idea
TODOZI void todozi_update_idea(const char* idea_id, void* updates);

// update_memory
TODOZI void todozi_update_memory(const char* memory_id, void* updates);

// update_project
TODOZI void todozi_update_project(todozi_project_t* project);

// update_registration_api_key
TODOZI void todozi_update_registration_api_key(const char* api_key);

// update_registration_keys
TODOZI void todozi_update_registration_keys(const char* api_key, const char* user_id, const char* fingerprint);

// update_reminder
TODOZI void todozi_update_reminder(const char* reminder_id, void* updates);

// update_summary
TODOZI void todozi_update_summary(const char* summary_id, void* updates);

// update_tag
TODOZI void todozi_update_tag(const char* tag_id, void* updates);

// update_task
TODOZI void todozi_update_task(const char* id, void* updates);

// update_task_full
TODOZI void todozi_update_task_full(const char* task_id, void* updates);

// update_task_in_project
TODOZI void todozi_update_task_in_project(const char* id, void* updates);

// update_task_status
TODOZI void todozi_update_task_status(const char* task_id, void* status);

// urgent
TODOZI const char* todozi_urgent(const char* action, todozi_error_t** error);

// validate_embeddings
TODOZI void* todozi_validate_embeddings(todozi_error_t** error);

// validate_migration
TODOZI bool todozi_validate_migration(todozi_error_t** error);

// validate_required_params
TODOZI void* todozi_validate_required_params(const char* kwargs, void* serde_json, const char* required_params, todozi_error_t** error);

// validate_string_param
TODOZI void* todozi_validate_string_param(const char* value, const char* param_name, size_t min_length, size_t max_length, const char* pattern, todozi_error_t** error);

// validate_task_input
TODOZI void todozi_validate_task_input(const char* action, const char* time, const char* priority, const char* project, const char* status, const char* assignee, void* progress);

// validate_tdz_command
TODOZI void* todozi_validate_tdz_command(const char* command, todozi_error_t** error);

// validation
TODOZI void* todozi_validation(void* message, todozi_error_t** error);

// verbose
TODOZI void* todozi_verbose(bool verbose, todozi_error_t** error);

// with_action
TODOZI void* todozi_with_action(const char* action, todozi_error_t** error);

// with_assignee
TODOZI void* todozi_with_assignee(void* assignee, todozi_error_t** error);

// with_context
TODOZI void* todozi_with_context(const char* context, todozi_error_t** error);

// with_context_notes
TODOZI void* todozi_with_context_notes(const char* context_notes, todozi_error_t** error);

// with_dependencies
TODOZI void* todozi_with_dependencies(void* dependencies, todozi_error_t** error);

// with_dry_run
TODOZI void* todozi_with_dry_run(bool dry_run, todozi_error_t** error);

// with_embedding_service
TODOZI void* todozi_with_embedding_service(void* service, todozi_error_t** error);

// with_embedding_service_option
TODOZI void* todozi_with_embedding_service_option(void* service, todozi_error_t** error);

// with_force
TODOZI void* todozi_with_force(bool force, todozi_error_t** error);

// with_max_tokens
TODOZI void* todozi_with_max_tokens(uint32_t max_tokens, todozi_error_t** error);

// with_parent_project
TODOZI void* todozi_with_parent_project(const char* parent_project, todozi_error_t** error);

// with_priority
TODOZI void* todozi_with_priority(void* priority, todozi_error_t** error);

// with_progress
TODOZI void* todozi_with_progress(void* progress, todozi_error_t** error);

// with_shared_components
TODOZI void* todozi_with_shared_components(void* config, void* cache, void* embedding_model, void* embedding_models, void* tag_manager, void* storage, todozi_error_t** error);

// with_status
TODOZI void* todozi_with_status(void* status, todozi_error_t** error);

// with_tags
TODOZI void* todozi_with_tags(void* tags, todozi_error_t** error);

// with_temperature
TODOZI void* todozi_with_temperature(float temperature, todozi_error_t** error);

// with_time
TODOZI void* todozi_with_time(const char* time, todozi_error_t** error);

// with_user_id
TODOZI void* todozi_with_user_id(const char* user_id, todozi_error_t** error);

// with_verbose
TODOZI void* todozi_with_verbose(bool verbose, todozi_error_t** error);

#ifdef __cplusplus
}
#endif

#endif // TODOZI_H
