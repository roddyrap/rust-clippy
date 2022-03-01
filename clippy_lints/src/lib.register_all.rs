// This file was generated by `cargo dev update_lints`.
// Use that command to update this file and do not edit by hand.
// Manual edits will be overwritten.

store.register_group(true, "clippy::all", Some("clippy_all"), vec![
    LintId::of(absurd_extreme_comparisons::ABSURD_EXTREME_COMPARISONS),
    LintId::of(approx_const::APPROX_CONSTANT),
    LintId::of(assertions_on_constants::ASSERTIONS_ON_CONSTANTS),
    LintId::of(assign_ops::ASSIGN_OP_PATTERN),
    LintId::of(assign_ops::MISREFACTORED_ASSIGN_OP),
    LintId::of(async_yields_async::ASYNC_YIELDS_ASYNC),
    LintId::of(attrs::BLANKET_CLIPPY_RESTRICTION_LINTS),
    LintId::of(attrs::DEPRECATED_CFG_ATTR),
    LintId::of(attrs::DEPRECATED_SEMVER),
    LintId::of(attrs::MISMATCHED_TARGET_OS),
    LintId::of(attrs::USELESS_ATTRIBUTE),
    LintId::of(await_holding_invalid::AWAIT_HOLDING_LOCK),
    LintId::of(await_holding_invalid::AWAIT_HOLDING_REFCELL_REF),
    LintId::of(bit_mask::BAD_BIT_MASK),
    LintId::of(bit_mask::INEFFECTIVE_BIT_MASK),
    LintId::of(blacklisted_name::BLACKLISTED_NAME),
    LintId::of(blocks_in_if_conditions::BLOCKS_IN_IF_CONDITIONS),
    LintId::of(bool_assert_comparison::BOOL_ASSERT_COMPARISON),
    LintId::of(booleans::LOGIC_BUG),
    LintId::of(booleans::NONMINIMAL_BOOL),
    LintId::of(casts::CAST_ENUM_TRUNCATION),
    LintId::of(casts::CAST_REF_TO_MUT),
    LintId::of(casts::CHAR_LIT_AS_U8),
    LintId::of(casts::FN_TO_NUMERIC_CAST),
    LintId::of(casts::FN_TO_NUMERIC_CAST_WITH_TRUNCATION),
    LintId::of(casts::UNNECESSARY_CAST),
    LintId::of(collapsible_if::COLLAPSIBLE_ELSE_IF),
    LintId::of(collapsible_if::COLLAPSIBLE_IF),
    LintId::of(collapsible_match::COLLAPSIBLE_MATCH),
    LintId::of(comparison_chain::COMPARISON_CHAIN),
    LintId::of(copies::IFS_SAME_COND),
    LintId::of(copies::IF_SAME_THEN_ELSE),
    LintId::of(default::FIELD_REASSIGN_WITH_DEFAULT),
    LintId::of(dereference::NEEDLESS_BORROW),
    LintId::of(derivable_impls::DERIVABLE_IMPLS),
    LintId::of(derive::DERIVE_HASH_XOR_EQ),
    LintId::of(derive::DERIVE_ORD_XOR_PARTIAL_ORD),
    LintId::of(disallowed_methods::DISALLOWED_METHODS),
    LintId::of(disallowed_types::DISALLOWED_TYPES),
    LintId::of(doc::MISSING_SAFETY_DOC),
    LintId::of(doc::NEEDLESS_DOCTEST_MAIN),
    LintId::of(double_comparison::DOUBLE_COMPARISONS),
    LintId::of(double_parens::DOUBLE_PARENS),
    LintId::of(drop_forget_ref::DROP_COPY),
    LintId::of(drop_forget_ref::DROP_REF),
    LintId::of(drop_forget_ref::FORGET_COPY),
    LintId::of(drop_forget_ref::FORGET_REF),
    LintId::of(duration_subsec::DURATION_SUBSEC),
    LintId::of(entry::MAP_ENTRY),
    LintId::of(enum_clike::ENUM_CLIKE_UNPORTABLE_VARIANT),
    LintId::of(enum_variants::ENUM_VARIANT_NAMES),
    LintId::of(enum_variants::MODULE_INCEPTION),
    LintId::of(eq_op::EQ_OP),
    LintId::of(eq_op::OP_REF),
    LintId::of(erasing_op::ERASING_OP),
    LintId::of(escape::BOXED_LOCAL),
    LintId::of(eta_reduction::REDUNDANT_CLOSURE),
    LintId::of(eval_order_dependence::DIVERGING_SUB_EXPRESSION),
    LintId::of(eval_order_dependence::EVAL_ORDER_DEPENDENCE),
    LintId::of(explicit_write::EXPLICIT_WRITE),
    LintId::of(float_equality_without_abs::FLOAT_EQUALITY_WITHOUT_ABS),
    LintId::of(float_literal::EXCESSIVE_PRECISION),
    LintId::of(format::USELESS_FORMAT),
    LintId::of(format_args::FORMAT_IN_FORMAT_ARGS),
    LintId::of(format_args::TO_STRING_IN_FORMAT_ARGS),
    LintId::of(format_impl::PRINT_IN_FORMAT_IMPL),
    LintId::of(format_impl::RECURSIVE_FORMAT_IMPL),
    LintId::of(formatting::POSSIBLE_MISSING_COMMA),
    LintId::of(formatting::SUSPICIOUS_ASSIGNMENT_FORMATTING),
    LintId::of(formatting::SUSPICIOUS_ELSE_FORMATTING),
    LintId::of(formatting::SUSPICIOUS_UNARY_OP_FORMATTING),
    LintId::of(from_over_into::FROM_OVER_INTO),
    LintId::of(from_str_radix_10::FROM_STR_RADIX_10),
    LintId::of(functions::DOUBLE_MUST_USE),
    LintId::of(functions::MUST_USE_UNIT),
    LintId::of(functions::NOT_UNSAFE_PTR_ARG_DEREF),
    LintId::of(functions::RESULT_UNIT_ERR),
    LintId::of(functions::TOO_MANY_ARGUMENTS),
    LintId::of(get_last_with_len::GET_LAST_WITH_LEN),
    LintId::of(identity_op::IDENTITY_OP),
    LintId::of(if_let_mutex::IF_LET_MUTEX),
    LintId::of(indexing_slicing::OUT_OF_BOUNDS_INDEXING),
    LintId::of(infinite_iter::INFINITE_ITER),
    LintId::of(inherent_to_string::INHERENT_TO_STRING),
    LintId::of(inherent_to_string::INHERENT_TO_STRING_SHADOW_DISPLAY),
    LintId::of(init_numbered_fields::INIT_NUMBERED_FIELDS),
    LintId::of(inline_fn_without_body::INLINE_FN_WITHOUT_BODY),
    LintId::of(int_plus_one::INT_PLUS_ONE),
    LintId::of(large_const_arrays::LARGE_CONST_ARRAYS),
    LintId::of(large_enum_variant::LARGE_ENUM_VARIANT),
    LintId::of(len_zero::COMPARISON_TO_EMPTY),
    LintId::of(len_zero::LEN_WITHOUT_IS_EMPTY),
    LintId::of(len_zero::LEN_ZERO),
    LintId::of(let_underscore::LET_UNDERSCORE_LOCK),
    LintId::of(lifetimes::EXTRA_UNUSED_LIFETIMES),
    LintId::of(lifetimes::NEEDLESS_LIFETIMES),
    LintId::of(literal_representation::INCONSISTENT_DIGIT_GROUPING),
    LintId::of(literal_representation::MISTYPED_LITERAL_SUFFIXES),
    LintId::of(literal_representation::UNUSUAL_BYTE_GROUPINGS),
    LintId::of(loops::EMPTY_LOOP),
    LintId::of(loops::EXPLICIT_COUNTER_LOOP),
    LintId::of(loops::FOR_KV_MAP),
    LintId::of(loops::FOR_LOOPS_OVER_FALLIBLES),
    LintId::of(loops::ITER_NEXT_LOOP),
    LintId::of(loops::MANUAL_FLATTEN),
    LintId::of(loops::MANUAL_MEMCPY),
    LintId::of(loops::MUT_RANGE_BOUND),
    LintId::of(loops::NEEDLESS_COLLECT),
    LintId::of(loops::NEEDLESS_RANGE_LOOP),
    LintId::of(loops::NEVER_LOOP),
    LintId::of(loops::SAME_ITEM_PUSH),
    LintId::of(loops::SINGLE_ELEMENT_LOOP),
    LintId::of(loops::WHILE_IMMUTABLE_CONDITION),
    LintId::of(loops::WHILE_LET_LOOP),
    LintId::of(loops::WHILE_LET_ON_ITERATOR),
    LintId::of(main_recursion::MAIN_RECURSION),
    LintId::of(manual_async_fn::MANUAL_ASYNC_FN),
    LintId::of(manual_bits::MANUAL_BITS),
    LintId::of(manual_map::MANUAL_MAP),
    LintId::of(manual_non_exhaustive::MANUAL_NON_EXHAUSTIVE),
    LintId::of(manual_strip::MANUAL_STRIP),
    LintId::of(manual_unwrap_or::MANUAL_UNWRAP_OR),
    LintId::of(map_clone::MAP_CLONE),
    LintId::of(map_unit_fn::OPTION_MAP_UNIT_FN),
    LintId::of(map_unit_fn::RESULT_MAP_UNIT_FN),
    LintId::of(match_result_ok::MATCH_RESULT_OK),
    LintId::of(match_str_case_mismatch::MATCH_STR_CASE_MISMATCH),
    LintId::of(matches::INFALLIBLE_DESTRUCTURING_MATCH),
    LintId::of(matches::MATCH_AS_REF),
    LintId::of(matches::MATCH_LIKE_MATCHES_MACRO),
    LintId::of(matches::MATCH_OVERLAPPING_ARM),
    LintId::of(matches::MATCH_REF_PATS),
    LintId::of(matches::MATCH_SINGLE_BINDING),
    LintId::of(matches::REDUNDANT_PATTERN_MATCHING),
    LintId::of(matches::SINGLE_MATCH),
    LintId::of(matches::WILDCARD_IN_OR_PATTERNS),
    LintId::of(mem_replace::MEM_REPLACE_OPTION_WITH_NONE),
    LintId::of(mem_replace::MEM_REPLACE_WITH_DEFAULT),
    LintId::of(mem_replace::MEM_REPLACE_WITH_UNINIT),
    LintId::of(methods::BIND_INSTEAD_OF_MAP),
    LintId::of(methods::BYTES_NTH),
    LintId::of(methods::CHARS_LAST_CMP),
    LintId::of(methods::CHARS_NEXT_CMP),
    LintId::of(methods::CLONE_DOUBLE_REF),
    LintId::of(methods::CLONE_ON_COPY),
    LintId::of(methods::EXPECT_FUN_CALL),
    LintId::of(methods::EXTEND_WITH_DRAIN),
    LintId::of(methods::FILTER_MAP_IDENTITY),
    LintId::of(methods::FILTER_NEXT),
    LintId::of(methods::FLAT_MAP_IDENTITY),
    LintId::of(methods::INSPECT_FOR_EACH),
    LintId::of(methods::INTO_ITER_ON_REF),
    LintId::of(methods::ITERATOR_STEP_BY_ZERO),
    LintId::of(methods::ITER_CLONED_COLLECT),
    LintId::of(methods::ITER_COUNT),
    LintId::of(methods::ITER_NEXT_SLICE),
    LintId::of(methods::ITER_NTH),
    LintId::of(methods::ITER_NTH_ZERO),
    LintId::of(methods::ITER_OVEREAGER_CLONED),
    LintId::of(methods::ITER_SKIP_NEXT),
    LintId::of(methods::MANUAL_FILTER_MAP),
    LintId::of(methods::MANUAL_FIND_MAP),
    LintId::of(methods::MANUAL_SATURATING_ARITHMETIC),
    LintId::of(methods::MANUAL_SPLIT_ONCE),
    LintId::of(methods::MANUAL_STR_REPEAT),
    LintId::of(methods::MAP_COLLECT_RESULT_UNIT),
    LintId::of(methods::MAP_FLATTEN),
    LintId::of(methods::MAP_IDENTITY),
    LintId::of(methods::NEEDLESS_SPLITN),
    LintId::of(methods::NEW_RET_NO_SELF),
    LintId::of(methods::OK_EXPECT),
    LintId::of(methods::OPTION_AS_REF_DEREF),
    LintId::of(methods::OPTION_FILTER_MAP),
    LintId::of(methods::OPTION_MAP_OR_NONE),
    LintId::of(methods::OR_FUN_CALL),
    LintId::of(methods::RESULT_MAP_OR_INTO_OPTION),
    LintId::of(methods::SEARCH_IS_SOME),
    LintId::of(methods::SHOULD_IMPLEMENT_TRAIT),
    LintId::of(methods::SINGLE_CHAR_ADD_STR),
    LintId::of(methods::SINGLE_CHAR_PATTERN),
    LintId::of(methods::SKIP_WHILE_NEXT),
    LintId::of(methods::STRING_EXTEND_CHARS),
    LintId::of(methods::SUSPICIOUS_MAP),
    LintId::of(methods::SUSPICIOUS_SPLITN),
    LintId::of(methods::UNINIT_ASSUMED_INIT),
    LintId::of(methods::UNNECESSARY_FILTER_MAP),
    LintId::of(methods::UNNECESSARY_FIND_MAP),
    LintId::of(methods::UNNECESSARY_FOLD),
    LintId::of(methods::UNNECESSARY_LAZY_EVALUATIONS),
    LintId::of(methods::UNNECESSARY_TO_OWNED),
    LintId::of(methods::UNWRAP_OR_ELSE_DEFAULT),
    LintId::of(methods::USELESS_ASREF),
    LintId::of(methods::WRONG_SELF_CONVENTION),
    LintId::of(methods::ZST_OFFSET),
    LintId::of(minmax::MIN_MAX),
    LintId::of(misc::CMP_NAN),
    LintId::of(misc::CMP_OWNED),
    LintId::of(misc::MODULO_ONE),
    LintId::of(misc::SHORT_CIRCUIT_STATEMENT),
    LintId::of(misc::TOPLEVEL_REF_ARG),
    LintId::of(misc::ZERO_PTR),
    LintId::of(misc_early::BUILTIN_TYPE_SHADOW),
    LintId::of(misc_early::DOUBLE_NEG),
    LintId::of(misc_early::DUPLICATE_UNDERSCORE_ARGUMENT),
    LintId::of(misc_early::MIXED_CASE_HEX_LITERALS),
    LintId::of(misc_early::REDUNDANT_PATTERN),
    LintId::of(misc_early::UNNEEDED_WILDCARD_PATTERN),
    LintId::of(misc_early::ZERO_PREFIXED_LITERAL),
    LintId::of(mut_key::MUTABLE_KEY_TYPE),
    LintId::of(mut_mutex_lock::MUT_MUTEX_LOCK),
    LintId::of(mut_reference::UNNECESSARY_MUT_PASSED),
    LintId::of(needless_arbitrary_self_type::NEEDLESS_ARBITRARY_SELF_TYPE),
    LintId::of(needless_bool::BOOL_COMPARISON),
    LintId::of(needless_bool::NEEDLESS_BOOL),
    LintId::of(needless_borrowed_ref::NEEDLESS_BORROWED_REFERENCE),
    LintId::of(needless_late_init::NEEDLESS_LATE_INIT),
    LintId::of(needless_option_as_deref::NEEDLESS_OPTION_AS_DEREF),
    LintId::of(needless_question_mark::NEEDLESS_QUESTION_MARK),
    LintId::of(needless_update::NEEDLESS_UPDATE),
    LintId::of(neg_cmp_op_on_partial_ord::NEG_CMP_OP_ON_PARTIAL_ORD),
    LintId::of(neg_multiply::NEG_MULTIPLY),
    LintId::of(new_without_default::NEW_WITHOUT_DEFAULT),
    LintId::of(no_effect::NO_EFFECT),
    LintId::of(no_effect::UNNECESSARY_OPERATION),
    LintId::of(non_copy_const::BORROW_INTERIOR_MUTABLE_CONST),
    LintId::of(non_copy_const::DECLARE_INTERIOR_MUTABLE_CONST),
    LintId::of(non_expressive_names::JUST_UNDERSCORES_AND_DIGITS),
    LintId::of(non_octal_unix_permissions::NON_OCTAL_UNIX_PERMISSIONS),
    LintId::of(octal_escapes::OCTAL_ESCAPES),
    LintId::of(open_options::NONSENSICAL_OPEN_OPTIONS),
    LintId::of(option_env_unwrap::OPTION_ENV_UNWRAP),
    LintId::of(overflow_check_conditional::OVERFLOW_CHECK_CONDITIONAL),
    LintId::of(partialeq_ne_impl::PARTIALEQ_NE_IMPL),
    LintId::of(precedence::PRECEDENCE),
    LintId::of(ptr::CMP_NULL),
    LintId::of(ptr::INVALID_NULL_PTR_USAGE),
    LintId::of(ptr::MUT_FROM_REF),
    LintId::of(ptr::PTR_ARG),
    LintId::of(ptr_eq::PTR_EQ),
    LintId::of(ptr_offset_with_cast::PTR_OFFSET_WITH_CAST),
    LintId::of(question_mark::QUESTION_MARK),
    LintId::of(ranges::MANUAL_RANGE_CONTAINS),
    LintId::of(ranges::RANGE_ZIP_WITH_LEN),
    LintId::of(ranges::REVERSED_EMPTY_RANGES),
    LintId::of(redundant_clone::REDUNDANT_CLONE),
    LintId::of(redundant_closure_call::REDUNDANT_CLOSURE_CALL),
    LintId::of(redundant_field_names::REDUNDANT_FIELD_NAMES),
    LintId::of(redundant_slicing::REDUNDANT_SLICING),
    LintId::of(redundant_static_lifetimes::REDUNDANT_STATIC_LIFETIMES),
    LintId::of(reference::DEREF_ADDROF),
    LintId::of(regex::INVALID_REGEX),
    LintId::of(repeat_once::REPEAT_ONCE),
    LintId::of(returns::LET_AND_RETURN),
    LintId::of(returns::NEEDLESS_RETURN),
    LintId::of(self_assignment::SELF_ASSIGNMENT),
    LintId::of(self_named_constructors::SELF_NAMED_CONSTRUCTORS),
    LintId::of(serde_api::SERDE_API_MISUSE),
    LintId::of(single_component_path_imports::SINGLE_COMPONENT_PATH_IMPORTS),
    LintId::of(size_of_in_element_count::SIZE_OF_IN_ELEMENT_COUNT),
    LintId::of(slow_vector_initialization::SLOW_VECTOR_INITIALIZATION),
    LintId::of(stable_sort_primitive::STABLE_SORT_PRIMITIVE),
    LintId::of(strings::STRING_FROM_UTF8_AS_BYTES),
    LintId::of(strlen_on_c_strings::STRLEN_ON_C_STRINGS),
    LintId::of(suspicious_trait_impl::SUSPICIOUS_ARITHMETIC_IMPL),
    LintId::of(suspicious_trait_impl::SUSPICIOUS_OP_ASSIGN_IMPL),
    LintId::of(swap::ALMOST_SWAPPED),
    LintId::of(swap::MANUAL_SWAP),
    LintId::of(tabs_in_doc_comments::TABS_IN_DOC_COMMENTS),
    LintId::of(temporary_assignment::TEMPORARY_ASSIGNMENT),
    LintId::of(to_digit_is_some::TO_DIGIT_IS_SOME),
    LintId::of(transmute::CROSSPOINTER_TRANSMUTE),
    LintId::of(transmute::TRANSMUTES_EXPRESSIBLE_AS_PTR_CASTS),
    LintId::of(transmute::TRANSMUTE_BYTES_TO_STR),
    LintId::of(transmute::TRANSMUTE_FLOAT_TO_INT),
    LintId::of(transmute::TRANSMUTE_INT_TO_BOOL),
    LintId::of(transmute::TRANSMUTE_INT_TO_CHAR),
    LintId::of(transmute::TRANSMUTE_INT_TO_FLOAT),
    LintId::of(transmute::TRANSMUTE_NUM_TO_BYTES),
    LintId::of(transmute::TRANSMUTE_PTR_TO_REF),
    LintId::of(transmute::TRANSMUTE_UNDEFINED_REPR),
    LintId::of(transmute::UNSOUND_COLLECTION_TRANSMUTE),
    LintId::of(transmute::WRONG_TRANSMUTE),
    LintId::of(transmuting_null::TRANSMUTING_NULL),
    LintId::of(try_err::TRY_ERR),
    LintId::of(types::BORROWED_BOX),
    LintId::of(types::BOX_COLLECTION),
    LintId::of(types::REDUNDANT_ALLOCATION),
    LintId::of(types::TYPE_COMPLEXITY),
    LintId::of(types::VEC_BOX),
    LintId::of(undropped_manually_drops::UNDROPPED_MANUALLY_DROPS),
    LintId::of(unicode::INVISIBLE_CHARACTERS),
    LintId::of(uninit_vec::UNINIT_VEC),
    LintId::of(unit_hash::UNIT_HASH),
    LintId::of(unit_return_expecting_ord::UNIT_RETURN_EXPECTING_ORD),
    LintId::of(unit_types::UNIT_ARG),
    LintId::of(unit_types::UNIT_CMP),
    LintId::of(unnamed_address::FN_ADDRESS_COMPARISONS),
    LintId::of(unnamed_address::VTABLE_ADDRESS_COMPARISONS),
    LintId::of(unnecessary_sort_by::UNNECESSARY_SORT_BY),
    LintId::of(unsafe_removed_from_name::UNSAFE_REMOVED_FROM_NAME),
    LintId::of(unused_io_amount::UNUSED_IO_AMOUNT),
    LintId::of(unused_unit::UNUSED_UNIT),
    LintId::of(unwrap::PANICKING_UNWRAP),
    LintId::of(unwrap::UNNECESSARY_UNWRAP),
    LintId::of(upper_case_acronyms::UPPER_CASE_ACRONYMS),
    LintId::of(useless_conversion::USELESS_CONVERSION),
    LintId::of(vec::USELESS_VEC),
    LintId::of(vec_init_then_push::VEC_INIT_THEN_PUSH),
    LintId::of(vec_resize_to_zero::VEC_RESIZE_TO_ZERO),
    LintId::of(write::PRINTLN_EMPTY_STRING),
    LintId::of(write::PRINT_LITERAL),
    LintId::of(write::PRINT_WITH_NEWLINE),
    LintId::of(write::WRITELN_EMPTY_STRING),
    LintId::of(write::WRITE_LITERAL),
    LintId::of(write::WRITE_WITH_NEWLINE),
    LintId::of(zero_div_zero::ZERO_DIVIDED_BY_ZERO),
])
