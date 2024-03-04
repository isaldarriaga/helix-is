use std::collections::HashMap;

use super::macros::keymap;
use super::{KeyTrie, Mode};
use helix_core::hashmap;

pub fn default() -> HashMap<Mode, KeyTrie> {
    let normal = keymap!({ "Normal mode"
        // "" => add_newline_above,
        // "" => add_newline_below,
        // "" => align_selections,
        // "" => align_view_bottom,
        // "" => align_view_center,
        // "" => align_view_middle,
        // "" => align_view_top,
        "a" => append_mode,
        "A-1" => buffer_picker,
        "c" => change_selection,
        "C" => change_selection_noyank,
        "A-ret" => code_action,
        // "" => collapse_selection,
        ":" => command_mode,
        ";" => command_palette,
        // "" => commit_undo_checkpoint,
        // "" => completion, // make sense in insert mode
        "C-S-down" => copy_selection_on_next_line, // multi-cursor
        "C-S-up" => copy_selection_on_prev_line, // multi-cursor
        // "F8" => dap_continue,
        // "" => dap_disable_exceptions,
        // "" => dap_edit_condition,
        // "" => dap_edit_log,
        // "" => dap_enable_exceptions,
        // "F5" => dap_launch,
        // "F10" => dap_next,
        // "" => dap_pause,
        // "C-S-F5" => dap_restart,
        // "F11" => dap_step_in,
        // "S-F11" => dap_step_out,
        // "" => dap_switch_stack_frame,
        // "" => dap_switch_thread,
        // "S-F5" => dap_terminate,
        // "F9" => dap_toggle_breakpoint,
        // "C-S-F9" => dap_variables,
        // "" => decrement,
        "backspace" => delete_char_backward,
        "del" => delete_char_forward,
        "C-x" | "d" => delete_selection, // cut
        "D" => delete_selection_noyank,
        "A-backspace" | "C-backspace" => delete_word_backward,
        "A-del" | "C-del" => delete_word_forward,
        "A-6" => diagnostics_picker,
        // "" => earlier,
        // "" => ensure_selections_forward,
        // "" => expand_selection,
        "S-left" => extend_char_left,
        "S-right" => extend_char_right,
        // "?" => extend_line,
        // "" => extend_line_above,
        // "" => extend_line_below,
        "S-down" => extend_line_down,
        "S-up" => extend_line_up,
        // "" => extend_next_char,
        // "" => extend_next_long_word_end,
        "C-S-A-right" => extend_next_long_word_start,
        // "" => extend_next_word_end,
        "C-S-right" => extend_next_word_start,
        "C-S-pagedown" | "S-pagedown" => extend_parent_node_end,
        "C-S-pageup" | "S-pageup" => extend_parent_node_start,
        // "" => extend_prev_char,
        "C-S-A-left" => extend_prev_long_word_end,
        // "" => extend_prev_long_word_start,
        "C-S-left" => extend_prev_word_end,
        // "" => extend_prev_word_start,
        "A-j" => extend_search_next,
        "A-S-j" => extend_search_prev,
        // "" => extend_till_char,
        // "" => extend_till_prev_char,
        "S-home" => extend_to_first_nonwhitespace,
        "X" => extend_to_line_bounds,
        // "" => extend_to_line_end,
        "S-end" => extend_to_line_end_newline,
        // "" => extend_to_line_start,
        // "" => extend_visual_line_down,
        // "" => extend_visual_line_up,
        // "" => file_picker, // seems redundant
        "C-<" | "C->" => file_picker_in_current_buffer_directory,
        "C-," | "C-." => file_picker_in_current_directory,
        "f" => find_next_char,
        "F" => find_prev_char,
        "t" => find_till_char,
        // "" => flip_selections,
        "C-t" => format_selections,
        "C-F" => global_search,
        "C-F12" => goto_declaration,
        "F12" => goto_definition,
        // "" => goto_file,
        "C-end" => goto_file_end,
        // "" => goto_file_hsplit,
        "C-home" => goto_file_start,
        // "" => goto_file_vsplit,
        // "" => goto_first_change,
        // "" => goto_first_diag,
        "home" => goto_first_nonwhitespace,
        // "" => goto_implementation,
        // "" => goto_last_accessed_file,
        // "" => goto_last_change,
        // "" => goto_last_diag,
        // "" => goto_last_line,
        "C-A-." => goto_last_modification,
        // "" => goto_last_modified_file,
        "C-g" => goto_line,
        // "" => goto_line_end,
        "end" => goto_line_end_newline,
        // "" => goto_line_start,
        "C-tab" => goto_next_buffer,
        "C-A-g" => goto_next_change, // g: git
        "C-A-c" => goto_next_class,
        "C-A-/" => goto_next_comment,
        "C-A-d" => goto_next_diag,
        "C-A-f" => goto_next_function,
        "C-A-p" => goto_next_paragraph,
        "C-A-a" => goto_next_parameter, // a: argument
        "C-A-t" => goto_next_test, // failed: try other lsp
        "C-A-G" => goto_prev_change,
        "C-A-C" => goto_prev_class,
        "C-A-S-/" => goto_prev_comment,
        "C-A-D" => goto_prev_diag,
        "C-A-F" => goto_prev_function,
        "C-A-P" => goto_prev_paragraph,
        "C-A-A" => goto_prev_parameter,
        "C-A-T" => goto_prev_test,
        "C-S-tab" => goto_previous_buffer,
        "S-F12" => goto_reference,
        // "F12" => goto_type_definition,
        // "" => goto_window_bottom,
        // "" => goto_window_center,
        // "" => goto_window_top,
        // "" => half_page_down,
        // "" => half_page_up,
        "F1" => hover,
        // "" => hsplit,
        // "" => hsplit_new,
        // "" => increment,
        "tab" => indent,
        "A" => insert_at_line_end,
        "I" => insert_at_line_start,
        "ins" | "i" => insert_mode,
        // "" => insert_newline,
        // "" => insert_register,
        // "tab" => insert_tab,
        // "" => join_selections, // put all in a single line
        // "" => join_selections_space,
        // "" => jump_backward,
        // "?" => jump_forward,
        // "?" => jump_view_down,
        // "" => jump_view_left,
        // "?" => jump_view_right,
        // "" => jump_view_up,
        "A-2" => jumplist_picker,
        // "" => keep_primary_selection,
        // "" => keep_selections,
        // "" => kill_to_line_end,
        // "" => kill_to_line_start,
        // "" => last_picker, // too weird
        // "" => later,
        // "" => make_search_word_bounded,
        // "" => match_brackets,
        // "" => merge_consecutive_selections,
        "A-minus" | "esc" => merge_selections, // cancel multi-cursor + keeps selection
        "left" => move_char_left,
        "right" => move_char_right,
        "down" => move_line_down,
        "up" => move_line_up,
        // "" => move_next_long_word_end,
        "C-A-right" => move_next_long_word_start,
        // "" => move_next_word_end,
        "C-right" | "w" => move_next_word_start,
        "C-pagedown" => move_parent_node_end,
        "C-pageup" => move_parent_node_start,
        "C-A-left" => move_prev_long_word_end,
        // "" => move_prev_long_word_start,
        "C-left" => move_prev_word_end,
        // "" => move_prev_word_start,
        // "?" => move_visual_line_down,
        // "" => move_visual_line_up,
        "O" => open_above,
        "o" => open_below,
        "pagedown" => page_cursor_down,
        // "" => page_cursor_half_down,
        // "" => page_cursor_half_up,
        "pageup" => page_cursor_up,
        //"" => page_down,
        //"" => page_up,
        //"p" => paste_after,
        //"P" => paste_before,
        "C-V" => paste_clipboard_after,
        "C-v" => paste_clipboard_before,
        // "?" => paste_primary_clipboard_after,
        // "" => paste_primary_clipboard_before,
        // "" => record_macro,
        "C-y" | "U" => redo,
        // "" => remove_primary_selection,
        // "" => remove_selections,
        "F2" => rename_symbol,
        // "" => repeat_last_motion, // too dangerous
        // "" => replace, // replaces with typed character (so weird)
        "C-r" => replace_selections_with_clipboard,
        // "?" => replace_selections_with_primary_clipboard,
        //"r" => replace_with_yanked,
        // "" => replay_macro,
        // "" => reverse_selection_contents,
        // "" => rotate_selection_contents_backward,
        // "" => rotate_selection_contents_forward,
        // "" => rotate_selections_backward,
        // "" => rotate_selections_forward,
        // "" => rotate_view,
        // "" => rotate_view_reverse,
        // "?" => rsearch,
        // "" => save_selection,
        "C-down" => scroll_down,
        "C-up" => scroll_up,
        "C-f" | "/" => search,
        "F3" | "n" => search_next,
        "S-F3" | "N" => search_prev,
        "C-F3" => search_selection,
        "C-a" => select_all,
        "v" | "s" => select_mode,
        // "C-S-A-right" => select_next_sibling, // short it
        // "" => select_prev_sibling,
        "h" => select_references_to_symbol_under_cursor, // h: highlight
        // "" => select_regex,
        // "" => select_register,
        // "" => select_textobject_around,
        // "" => select_textobject_inner,
        // "" => shell_append_output,
        // "" => shell_insert_output,
        // "" => shell_keep_pipe,
        // "" => shell_pipe,
        // "" => shell_pipe_to,
        // "" => shrink_selection,
        // "" => shrink_to_line_bounds, // oposite to Shift + Xi (seems not needed)
        // "?" => signature_help, // fail
        // "" => smart_tab,
        // "" => split_selection,
        // "" => split_selection_on_newline,
        // "" => surround_add,
        // "" => surround_delete,
        // "" => surround_replace,
        // "" => suspend,
        // "" => swap_view_down,
        // "" => swap_view_left,
        // "?" => swap_view_right,
        // "" => swap_view_up,
        "C-A-u" => switch_case,
        "C-l" => switch_to_lowercase,
        "C-u" => switch_to_uppercase,
        // "" => symbol_picker, // i dont like this
        "T" => till_prev_char,
        // "" => toggle_block_comments,
        "C-/" => toggle_comments,
        // "" => toggle_line_comments,
        // "" => transpose_view,
        // "" => trim_selections,
        "C-z" | "u" => undo,
        "S-tab" => unindent,
        // "" => vsplit,
        // "" => vsplit_new,
        // "" => wclose,
        // "" => wonly,
        "A-^" => workspace_diagnostics_picker, // Alt + Shift + 6
        // "" => workspace_symbol_picker,
        //"y" => yank,
        // "" => yank_joined,
        // "" => yank_joined_to_clipboard,
        // "" => yank_joined_to_primary_clipboard,
        // "" => yank_main_selection_to_clipboard,
        // "" => yank_main_selection_to_primary_clipboard,
        "C-c" => yank_to_clipboard,
        // "" => yank_to_primary_clipboard,

        "space" => {" ⭐ Space "
            "┈" => menu_divider,
        },

        "A-." => {" 🧬 Main Menu "
            "f" => { "📂 File  " sticky=false
                "n" => { "  New  " sticky=false
                    "f" => menu_new_file,
                    "s" => menu_new_scratch_file,
                    "d" => menu_new_directory_at_buffer_path,
                    "┈" => menu_divider,
                },
                "o" => { "  Open  " sticky=false
                    "C-," | "C-." => file_picker_in_current_directory,
                    "C-<" | "C->" => file_picker_in_current_buffer_directory,
                },
            },

            "e" => { "📝️  Edit  " sticky=true
                "C-z" | "u" => undo,
                "C-y" | "U" => redo,
                "┈" => menu_divider,

                "y" => { "📤 Yank  " sticky=true
                    "y" => yank,
                    "j" => yank_joined,
                    "P" => paste_before,
                    "p" => paste_after,
                    "r" => replace_with_yanked,
                },

                "C" => { "📋 Clipboard  " sticky=true
                    "C-c" => yank_to_clipboard,
                    "C-C" => yank_joined_to_clipboard,
                    // "" => yank_main_selection_to_clipboard, // so weird
                    "C-v" => paste_clipboard_after,
                    "C-V" => paste_clipboard_before,
                    "C-r" => replace_selections_with_clipboard,
                },

                "P" => { "🎬 Primary Clipboard  " sticky=true
                    "esc" => normal_mode,
                    //"" => yank_to_primary_clipboard, // what is this?
                    //"" => yank_joined_to_primary_clipboard,
                    //"" => yank_main_selection_to_primary_clipboard,
                    //"" => paste_primary_clipboard_before,
                    //"" => paste_primary_clipboard_after,
                    //"" => replace_selections_with_primary_clipboard,
                },
                "." => menu_divider,
                "F1" => hover,
                "F2" => rename_symbol,
                "A-ret" => code_action,
                "h" => select_references_to_symbol_under_cursor, // h: highlight
            },

            "v" => { "🧿 View  "
                "t" => { "  Tool Windows  " sticky=false
                    "A-1" => buffer_picker,
                    "A-2" => jumplist_picker,
                    "A-6" => diagnostics_picker,
                    "┈" => menu_divider,
                    "A-^" => workspace_diagnostics_picker,
                    "s" => symbol_picker,
                    "S" => workspace_symbol_picker,
                },
            },

            "n" => { "⛵ Navigate  "
                "esc" => normal_mode,
            },

            "c" => { "💻 Code  "
                "esc" => normal_mode,
            },

            "r" => { "🔧 Refactor  "
                "esc" => normal_mode,
            },

            "u" => { "▶️ Run  "
                "d" => { "🐞 Debug  " sticky=true
                    "esc" => normal_mode,
                    //"" => dap_launch,
                    //"" => dap_restart,
                    //"" => dap_toggle_breakpoint,
                    //"" => dap_continue,
                    //"" => dap_pause,
                    //"" => dap_step_in,
                    //"" => dap_step_out,
                    //"" => dap_next,
                    //"" => dap_variables,
                    //"" => dap_terminate,
                    //"" => dap_edit_condition,
                    //"" => dap_edit_log,
                    //"" => { " Switch  "
                    //    "" => dap_switch_thread,
                    //    "" => dap_switch_stack_frame,
                    //},
                    //"" => dap_enable_exceptions,
                    //"" => dap_disable_exceptions,
                },
            },

            "t" => { "🧰 Tools  " sticky=true
                "esc" => normal_mode,
            },

            "g" => { " Git  " sticky=true
                "esc" => normal_mode,
            },

            "w" => { "🪟 Window  "
                "esc" => normal_mode,
                //"" => rotate_view,
                //"" => hsplit,
                //"" => vsplit,
                //"" => transpose_view,
                //"" => goto_file_hsplit,
                //"" => goto_file_vsplit,
                //"" => wclose,
                //"" => wonly,
                //"" => jump_view_left,
                //"" => jump_view_down,
                //"" => jump_view_up,
                //"?" => jump_view_right,
                //"" => swap_view_left,
                //"" => swap_view_down,
                //"" => swap_view_up,
                //"?" => swap_view_right,
                //"" => { "New split scratch buffer"
                //    "" => hsplit_new,
                //    "" => vsplit_new,
                //},
            },

            "h" => { "❓ Help  " sticky=true
                "esc" => normal_mode,
            },
        },

        // Menu (normal mode)
        "g" => { " 🚀 Goto "
            "esc" => normal_mode,
            //"" => goto_first_nonwhitespace,
            //"" => goto_definition,
            //"" => goto_declaration,
            //"" => goto_type_definition,
            //"" => goto_reference,
            //"" => goto_implementation,
            //"" => goto_window_top,
            //"" => goto_window_center,
            //"" => goto_window_bottom,
            //"" => goto_last_accessed_file,
            //"" => goto_last_modified_file,
            //"" => goto_last_modification,
        },

        // Menu (normal mode)
        "m" => { " 󰾹 Match "
            "esc" => normal_mode,
            //"" => match_brackets,
            //"" => surround_add,
            //"" => surround_replace,
            //"" => surround_delete,
            //"" => select_textobject_around,
            //"" => select_textobject_inner,
        },

        // Menu (normal mode)
        "[" => { " ⮬ Goto "
            "esc" => normal_mode,
            //"" => goto_first_diag,
            //"" => goto_first_change,
            //"" => goto_prev_diag,
            //"" => goto_prev_change,
            //"" => goto_prev_function,
            //"" => goto_prev_class,
            //"" => goto_prev_parameter,
            //"" => goto_prev_comment,
            //"" => goto_prev_test,
            //"" => goto_prev_paragraph,
        },

        // Menu (normal mode)
        "]" => { " Goto ⮯ "
            "esc" => normal_mode,
            //"" => goto_next_diag,
            //"" => goto_next_change,
            //"" => goto_next_function,
            //"" => goto_next_class,
            //"" => goto_next_parameter,
            //"" => goto_next_comment,
            //"" => goto_next_test,
            //"" => goto_next_paragraph,
            //"" => goto_last_diag,
            //"" => goto_last_change,
        },
    });

    let mut select = normal.clone();
    select.merge_nodes(keymap!({ "Select mode"
        "esc" => exit_select_mode,
        // copy non-detected normal mode commands here

        // Menu (select mode)
        "g" => { "Goto"
            "esc" => exit_select_mode,
            //"" => extend_line_up,
            //"" => extend_line_down,
        },
    }));

    let insert = keymap!({ "Insert mode"
        "esc" => normal_mode,
        // "" => add_newline_above,
        // "" => add_newline_below,
        // "" => align_selections,
        // "" => align_view_bottom,
        // "" => align_view_center,
        // "" => align_view_middle,
        // "" => align_view_top,
        // "a" => append_mode,
        "A-1" => buffer_picker,
        // "c" => change_selection,
        // "C" => change_selection_noyank,
        "A-ret" => code_action,
        // "" => collapse_selection,
        // ":" => command_mode,
        // ";" => command_palette,
        // "" => commit_undo_checkpoint,
        "C-space" => completion,
        "C-S-down" => copy_selection_on_next_line, // multi-cursor
        "C-S-up" => copy_selection_on_prev_line, // multi-cursor
        // "F8" => dap_continue,
        // "" => dap_disable_exceptions,
        // "" => dap_edit_condition,
        // "" => dap_edit_log,
        // "" => dap_enable_exceptions,
        // "F5" => dap_launch,
        // "F10" => dap_next,
        // "" => dap_pause,
        // "C-S-F5" => dap_restart,
        // "F11" => dap_step_in,
        // "S-F11" => dap_step_out,
        // "" => dap_switch_stack_frame,
        // "" => dap_switch_thread,
        // "S-F5" => dap_terminate,
        // "F9" => dap_toggle_breakpoint,
        // "C-S-F9" => dap_variables,
        // "" => decrement,
        "backspace" => delete_char_backward,
        "del" => delete_char_forward,
        "C-x" | "d" => delete_selection, // cut
        // "D" => delete_selection_noyank,
        "A-backspace" | "C-backspace" => delete_word_backward,
        "A-del" | "C-del" => delete_word_forward,
        "A-6" => diagnostics_picker,
        // "" => earlier,
        // "" => ensure_selections_forward,
        // "" => expand_selection,
        "S-left" => extend_char_left,
        "S-right" => extend_char_right,
        // "?" => extend_line,
        // "" => extend_line_above,
        // "" => extend_line_below,
        "S-down" => extend_line_down,
        "S-up" => extend_line_up,
        // "" => extend_next_char,
        // "" => extend_next_long_word_end,
        "C-S-A-right" => extend_next_long_word_start,
        // "" => extend_next_word_end,
        "C-S-right" => extend_next_word_start,
        "C-S-pagedown" | "S-pagedown" => extend_parent_node_end,
        "C-S-pageup" | "S-pageup" => extend_parent_node_start,
        // "" => extend_prev_char,
        "C-S-A-left" => extend_prev_long_word_end,
        // "" => extend_prev_long_word_start,
        "C-S-left" => extend_prev_word_end,
        // "" => extend_prev_word_start,
        // "" => extend_search_next,
        // "" => extend_search_prev,
        // "" => extend_till_char,
        // "" => extend_till_prev_char,
        "S-home" => extend_to_first_nonwhitespace,
        // "X" => extend_to_line_bounds,
        // "" => extend_to_line_end,
        "S-end" => extend_to_line_end_newline,
        // "" => extend_to_line_start,
        // "" => extend_visual_line_down,
        // "" => extend_visual_line_up,
        // "" => file_picker, // seems redundant
        "C-<" | "C->" => file_picker_in_current_buffer_directory,
        "C-," | "C-." => file_picker_in_current_directory,
        // "f" => find_next_char,
        // "F" => find_prev_char,
        // "t" => find_till_char,
        // "" => flip_selections,
        "C-t" => format_selections,
        "C-F" => global_search,
        "C-F12" => goto_declaration,
        "F12" => goto_definition,
        // "" => goto_file,
        "C-end" => goto_file_end,
        // "" => goto_file_hsplit,
        "C-home" => goto_file_start,
        // "" => goto_file_vsplit,
        // "" => goto_first_change,
        // "" => goto_first_diag,
        "home" => goto_first_nonwhitespace,
        // "" => goto_implementation,
        // "" => goto_last_accessed_file,
        // "" => goto_last_change,
        // "" => goto_last_diag,
        // "" => goto_last_line,
        "C-A-." => goto_last_modification,
        // "" => goto_last_modified_file,
        "C-g" => goto_line,
        // "" => goto_line_end,
        "end" => goto_line_end_newline,
        // "" => goto_line_start,
        "C-tab" => goto_next_buffer,
        "C-A-g" => goto_next_change, // g: git
        "C-A-c" => goto_next_class,
        "C-A-/" => goto_next_comment,
        "C-A-d" => goto_next_diag,
        "C-A-f" => goto_next_function,
        "C-A-p" => goto_next_paragraph,
        "C-A-a" => goto_next_parameter, // a: argument
        "C-A-t" => goto_next_test, // failed: try other lsp
        "C-A-G" => goto_prev_change,
        "C-A-C" => goto_prev_class,
        "C-A-S-/" => goto_prev_comment,
        "C-A-D" => goto_prev_diag,
        "C-A-F" => goto_prev_function,
        "C-A-P" => goto_prev_paragraph,
        "C-A-A" => goto_prev_parameter,
        "C-A-T" => goto_prev_test,
        "C-S-tab" => goto_previous_buffer,
        "S-F12" => goto_reference,
        // "F12" => goto_type_definition,
        // "" => goto_window_bottom,
        // "" => goto_window_center,
        // "" => goto_window_top,
        // "" => half_page_down,
        // "" => half_page_up,
        "F1" => hover,
        // "" => hsplit,
        // "" => hsplit_new,
        // "" => increment,
        // "tab" => indent,
        // "A" => insert_at_line_end,
        // "I" => insert_at_line_start,
        // "ins" | "i" => insert_mode,
        // "" => insert_newline,
        // "" => insert_register,
        "tab" => insert_tab,
        // "" => join_selections, // put all in a single line
        // "" => join_selections_space,
        // "" => jump_backward,
        // "?" => jump_forward,
        // "?" => jump_view_down,
        // "" => jump_view_left,
        // "?" => jump_view_right,
        // "" => jump_view_up,
        "A-2" => jumplist_picker,
        // "" => keep_primary_selection,
        // "" => keep_selections,
        // "" => kill_to_line_end,
        // "" => kill_to_line_start,
        // "" => last_picker, // too weird
        // "" => later,
        // "" => make_search_word_bounded,
        // "" => match_brackets,
        // "" => merge_consecutive_selections,
        "A-minus" => merge_selections, // cancel multi-cursor + keeps selection
        "left" => move_char_left,
        "right" => move_char_right,
        "down" => move_line_down,
        "up" => move_line_up,
        // "" => move_next_long_word_end,
        "C-A-right" => move_next_long_word_start,
        // "" => move_next_word_end,
        "C-right" => move_next_word_start,
        "C-pagedown" => move_parent_node_end,
        "C-pageup" => move_parent_node_start,
        "C-A-left" => move_prev_long_word_end,
        // "" => move_prev_long_word_start,
        "C-left" => move_prev_word_end,
        // "" => move_prev_word_start,
        // "?" => move_visual_line_down,
        // "" => move_visual_line_up,
        // "O" => open_above,
        // "o" => open_below,
        "pagedown" => page_cursor_down,
        // "" => page_cursor_half_down,
        // "" => page_cursor_half_up,
        "pageup" => page_cursor_up,
        //"" => page_down,
        //"" => page_up,
        //"p" => paste_after,
        //"P" => paste_before,
        "C-V" => paste_clipboard_after,
        "C-v" => paste_clipboard_before,
        // "?" => paste_primary_clipboard_after,
        // "" => paste_primary_clipboard_before,
        // "" => record_macro,
        "C-y" => redo,
        // "" => remove_primary_selection,
        // "" => remove_selections,
        "F2" => rename_symbol,
        // "" => repeat_last_motion, // too dangerous
        // "" => replace, // replaces with typed character (so weird)
        "C-r" => replace_selections_with_clipboard,
        // "?" => replace_selections_with_primary_clipboard,
        //"r" => replace_with_yanked,
        // "" => replay_macro,
        // "" => reverse_selection_contents,
        // "" => rotate_selection_contents_backward,
        // "" => rotate_selection_contents_forward,
        // "" => rotate_selections_backward,
        // "" => rotate_selections_forward,
        // "" => rotate_view,
        // "" => rotate_view_reverse,
        // "?" => rsearch,
        // "" => save_selection,
        "C-down" => scroll_down,
        "C-up" => scroll_up,
        "C-f" => search,
        "F3" => search_next,
        "S-F3" => search_prev,
        "C-F3" => search_selection,
        "C-a" => select_all,
        // "v" | "s" => select_mode,
        // "C-S-A-right" => select_next_sibling, // short it
        // "" => select_prev_sibling,
        // "h" => select_references_to_symbol_under_cursor, // h: highlight
        // "" => select_regex,
        // "" => select_register,
        // "" => select_textobject_around,
        // "" => select_textobject_inner,
        // "" => shell_append_output,
        // "" => shell_insert_output,
        // "" => shell_keep_pipe,
        // "" => shell_pipe,
        // "" => shell_pipe_to,
        // "" => shrink_selection,
        // "" => shrink_to_line_bounds, // oposite to Shift + Xi (seems not needed)
        // "?" => signature_help, // fail
        // "" => smart_tab,
        // "" => split_selection,
        // "" => split_selection_on_newline,
        // "" => surround_add,
        // "" => surround_delete,
        // "" => surround_replace,
        // "" => suspend,
        // "" => swap_view_down,
        // "" => swap_view_left,
        // "?" => swap_view_right,
        // "" => swap_view_up,
        "C-A-u" => switch_case,
        "C-l" => switch_to_lowercase,
        "C-u" => switch_to_uppercase,
        // "" => symbol_picker, // i dont like this
        // "T" => till_prev_char,
        // "" => toggle_block_comments,
        "C-/" => toggle_comments,
        // "" => toggle_line_comments,
        // "" => transpose_view,
        // "" => trim_selections,
        "C-z" => undo,
        "S-tab" => unindent,
        // "" => vsplit,
        // "" => vsplit_new,
        // "" => wclose,
        // "" => wonly,
        "A-^" => workspace_diagnostics_picker, // Alt + Shift + 6
        // "" => workspace_symbol_picker,
        //"y" => yank,
        // "" => yank_joined,
        // "" => yank_joined_to_clipboard,
        // "" => yank_joined_to_primary_clipboard,
        // "" => yank_main_selection_to_clipboard,
        // "" => yank_main_selection_to_primary_clipboard,
        "C-c" => yank_to_clipboard,
        // "" => yank_to_primary_clipboard,

    });
    hashmap!(
        Mode::Normal => normal,
        Mode::Select => select,
        Mode::Insert => insert,
    )
}
