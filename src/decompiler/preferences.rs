/// Name: Remove Bridge Methods<br>
/// Description: Removes any methods that are marked as bridge from the decompiled output.<br>
/// Short Name: rbr<br>
/// Type: Boolean
pub const REMOVE_BRIDGE: &str = "remove-bridge";

/// Name: Remove Synthetic Methods And Fields<br>
/// Description: Removes any methods and fields that are marked as synthetic from the decompiled output.<br>
/// Short Name: rsy<br>
/// Type: Boolean
pub const REMOVE_SYNTHETIC: &str = "remove-synthetic";

/// Name: Decompile Inner Classes<br>
/// Description: Process inner classes and add them to the decompiled output.<br>
/// Short Name: din<br>
/// Type: Boolean
pub const DECOMPILE_INNER: &str = "decompile-inner";

/// Name: Decompile Java 4 class references<br>
/// Description: Resugar the Java 1-4 class reference format instead of leaving the synthetic code.<br>
/// Short Name: dc4<br>
/// Type: Boolean
pub const DECOMPILE_CLASS_1_4: &str = "decompile-java4";

/// Name: Decompile Assertions<br>
/// Description: Decompile assert statements.<br>
/// Short Name: das<br>
/// Type: Boolean
pub const DECOMPILE_ASSERTIONS: &str = "decompile-assert";

/// Name: Hide Empty super()<br>
/// Description: Hide super() calls with no parameters.<br>
/// Short Name: hes<br>
/// Type: Boolean
pub const HIDE_EMPTY_SUPER: &str = "hide-empty-super";

/// Name: Hide Default Constructor<br>
/// Description: Hide constructors with no parameters and no code.<br>
/// Short Name: hdc<br>
/// Type: Boolean
pub const HIDE_DEFAULT_CONSTRUCTOR: &str = "hide-default-constructor";

/// Name: Decompile Generics<br>
/// Description: Decompile generics in classes, methods, fields, and variables.<br>
/// Short Name: dgs<br>
/// Type: Boolean
pub const DECOMPILE_GENERIC_SIGNATURES: &str = "decompile-generics";

/// Name: Incorporate returns in try-catch blocks<br>
/// Description: Integrate returns better in try-catch blocks instead of storing them in a temporary variable.<br>
/// Short Name: ner<br>
/// Type: Boolean
pub const INCORPORATE_RETURNS: &str = "incorporate-returns";

/// Name: Ensure synchronized ranges are complete<br>
/// Description: If a synchronized block has a monitorenter without any corresponding monitorexit, try to deduce where one should be to ensure the synchronized is correctly decompiled.<br>
/// Short Name: esm<br>
/// Type: Boolean
pub const ENSURE_SYNCHRONIZED_MONITOR: &str = "ensure-synchronized-monitors";

/// Name: Decompile Enums<br>
/// Description: Decompile enums.<br>
/// Short Name: den<br>
/// Type: Boolean
pub const DECOMPILE_ENUM: &str = "decompile-enums";

/// Name: Decompile Preview Features<br>
/// Description: Decompile features marked as preview or incubating in the latest Java versions.<br>
/// Short Name: dpr<br>
/// Type: Boolean
pub const DECOMPILE_PREVIEW: &str = "decompile-preview";

/// Name: Remove reference getClass()<br>
/// Description: Remove synthetic getClass() calls created by code such as 'obj.new Inner()'.<br>
/// Short Name: rgn<br>
/// Type: Boolean
pub const REMOVE_GET_CLASS_NEW: &str = "remove-getclass";

/// Name: Keep Literals As Is<br>
/// Description: Keep NaN, infinities, and pi values as is without resugaring them.<br>
/// Short Name: lit<br>
/// Type: Boolean
pub const LITERALS_AS_IS: &str = "keep-literals";

/// Name: Represent boolean as 0/1<br>
/// Description: Represent integers 0 and 1 as booleans.<br>
/// Short Name: bto<br>
/// Type: Boolean
pub const BOOLEAN_TRUE_ONE: &str = "boolean-as-int";

/// Name: ASCII String Characters<br>
/// Description: Encode non-ASCII characters in string and character literals as Unicode escapes.<br>
/// Short Name: asc<br>
/// Type: Boolean
pub const ASCII_STRING_CHARACTERS: &str = "ascii-strings";

/// Name: Synthetic Not Set<br>
/// Description: Treat some known structures as synthetic even when not explicitly set.<br>
/// Short Name: nns<br>
/// Type: Boolean
pub const SYNTHETIC_NOT_SET: &str = "synthetic-not-set";

/// Name: Treat Undefined Param Type As Object<br>
/// Description: Treat nameless types as java.lang.Object.<br>
/// Short Name: uto<br>
/// Type: Boolean
pub const UNDEFINED_PARAM_TYPE_OBJECT: &str = "undefined-as-object";

/// Name: Use LVT Names<br>
/// Description: Use LVT names for local variables and parameters instead of var<index>_<version>.<br>
/// Short Name: udv<br>
/// Type: Boolean
pub const USE_DEBUG_VAR_NAMES: &str = "use-lvt-names";

/// Name: Use Method Parameters<br>
/// Description: Use method parameter names, as given in the MethodParameters attribute.<br>
/// Short Name: ump<br>
/// Type: Boolean
pub const USE_METHOD_PARAMETERS: &str = "use-method-parameters";

/// Name: Remove Empty try-catch blocks<br>
/// Description: Remove try-catch blocks with no code.<br>
/// Short Name: rer<br>
/// Type: Boolean
pub const REMOVE_EMPTY_RANGES: &str = "remove-empty-try-catch";

/// Name: Decompile Finally<br>
/// Description: Decompile finally blocks.<br>
/// Short Name: fdi<br>
/// Type: Boolean
pub const FINALLY_DEINLINE: &str = "decompile-finally";

/// Name: Decompile Lambdas as Anonymous Classes<br>
/// Description: Decompile lambda expressions as anonymous classes.<br>
/// Short Name: lac<br>
/// Type: Boolean
pub const LAMBDA_TO_ANONYMOUS_CLASS: &str = "lambda-to-anonymous-class";

/// Name: Bytecode to Source Mapping<br>
/// Description: Map Bytecode to source lines.<br>
/// Short Name: bsm<br>
/// Type: Boolean
pub const BYTECODE_SOURCE_MAPPING: &str = "bytecode-source-mapping";

/// Name: Dump Code Lines<br>
/// Description: Dump line mappings to output archive zip entry extra data.<br>
/// Short Name: dcl<br>
/// Type: Boolean
pub const DUMP_CODE_LINES: &str = "dump-code-lines";

/// Name: Ignore Invalid Bytecode<br>
/// Description: Ignore bytecode that is malformed.<br>
/// Short Name: iib<br>
/// Type: Boolean
pub const IGNORE_INVALID_BYTECODE: &str = "ignore-invalid-bytecode";

/// Name: Verify Anonymous Classes<br>
/// Description: Verify that anonymous classes are local.<br>
/// Short Name: vac<br>
/// Type: Boolean
pub const VERIFY_ANONYMOUS_CLASSES: &str = "verify-anonymous-classes";

/// Name: Ternary Constant Simplification<br>
/// Description: Fold branches of ternary expressions that have boolean true and false constants.<br>
/// Short Name: tcs<br>
/// Type: Boolean
pub const TERNARY_CONSTANT_SIMPLIFICATION: &str = "ternary-constant-simplification";

/// Name: Pattern Matching<br>
/// Description: Decompile with if and switch pattern matching enabled.<br>
/// Short Name: pam<br>
/// Type: Boolean
pub const PATTERN_MATCHING: &str = "pattern-matching";

/// Name: Try-Loop fix<br>
/// Description: Fixes rare cases of malformed decompilation when try blocks are found inside of while loops<br>
/// Short Name: tlf<br>
/// Type: Boolean
pub const TRY_LOOP_FIX: &str = "try-loop-fix";

/// Name: [Experimental] Ternary In If Conditions<br>
/// Description: Tries to collapse if statements that have a ternary in their condition.<br>
/// Short Name: tco<br>
/// Type: Boolean
pub const TERNARY_CONDITIONS: &str = "ternary-in-if";

/// Name: Decompile Switch Expressions<br>
/// Description: Decompile switch expressions in modern Java class files.<br>
/// Short Name: swe<br>
/// Type: Boolean
pub const SWITCH_EXPRESSIONS: &str = "decompile-switch-expressions";

/// Name: [Debug] Show hidden statements<br>
/// Description: Display hidden code blocks for debugging purposes.<br>
/// Short Name: shs<br>
/// Type: Boolean
pub const SHOW_HIDDEN_STATEMENTS: &str = "show-hidden-statements";

/// Name: Override Annotation<br>
/// Description: Display override annotations for methods known to the decompiler.<br>
/// Short Name: ovr<br>
/// Type: Boolean
pub const OVERRIDE_ANNOTATION: &str = "override-annotation";

/// Name: Second-Pass Stack Simplification<br>
/// Description: Simplify variables across stack bounds to resugar complex statements.<br>
/// Short Name: ssp<br>
/// Type: Boolean
pub const SIMPLIFY_STACK_SECOND_PASS: &str = "simplify-stack";

/// Name: [Experimental] Verify Variable Merges<br>
/// Description: Tries harder to verify the validity of variable merges. If there are strange variable recompilation issues, this is a good place to start.<br>
/// Short Name: vvm<br>
/// Type: Boolean
pub const VERIFY_VARIABLE_MERGES: &str = "verify-merges";

/// Name: [Experimental] Use old try deduplication<br>
/// Description: Use the old try deduplication algorithm for methods with obfuscated exceptions, which inserts dummy exception handlers instead of duplicating blocks<br>
/// Type: Boolean
pub const OLD_TRY_DEDUP: &str = "old-try-dedup";

/// Name: Include Entire Classpath<br>
/// Description: Give the decompiler information about every jar on the classpath.<br>
/// Short Name: iec<br>
/// Type: Boolean
pub const INCLUDE_ENTIRE_CLASSPATH: &str = "include-classpath";

/// Name: Include Java Runtime<br>
/// Description: Give the decompiler information about the Java runtime, either 1 or current for the current runtime, or a path to another runtime<br>
/// Short Name: jrt<br>
/// Type: String
pub const INCLUDE_JAVA_RUNTIME: &str = "include-runtime";

/// Name: Explicit Generic Arguments<br>
/// Description: Put explicit diamond generic arguments on method calls.<br>
/// Short Name: ega<br>
/// Type: Boolean
pub const EXPLICIT_GENERIC_ARGUMENTS: &str = "explicit-generics";

/// Name: Inline Simple Lambdas<br>
/// Description: Remove braces on simple, one line, lambda expressions.<br>
/// Short Name: isl<br>
/// Type: Boolean
pub const INLINE_SIMPLE_LAMBDAS: &str = "inline-simple-lambdas";

/// Name: Logging Level<br>
/// Description: Logging level. Must be one of: 'info', 'debug', 'warn', 'error'.<br>
/// Short Name: log<br>
/// Type: String
pub const LOG_LEVEL: &str = "log-level";

/// Name: [DEPRECATED] Max time to process method<br>
/// Description: Maximum time in seconds to process a method. This is deprecated, do not use.<br>
/// Short Name: mpm<br>
/// Type: Integer
pub const MAX_PROCESSING_METHOD: &str = "max-time-per-method";

/// Name: Rename Members<br>
/// Description: Rename classes, fields, and methods with a number suffix to help in deobfuscation.<br>
/// Short Name: ren<br>
/// Type: Boolean
pub const RENAME_ENTITIES: &str = "rename-members";

/// Name: User Renamer Class<br>
/// Description: Path to a class that implements IIdentifierRenamer.<br>
/// Short Name: urc<br>
/// Type: String
pub const USER_RENAMER_CLASS: &str = "user-renamer-class";

/// Name: [DEPRECATED] New Line Seperator<br>
/// Description: Use \\n instead of \\r\\n for new lines. Deprecated, do not use.<br>
/// Short Name: nls<br>
/// Type: Boolean<br>
/// Dynamic Default Value: Disabled on Windows, enabled on other systems
pub const NEW_LINE_SEPARATOR: &str = "new-line-separator";

/// Name: Indent String<br>
/// Description: A string of spaces or tabs that is placed for each indent level.<br>
/// Short Name: ind<br>
/// Type: String
pub const INDENT_STRING: &str = "indent-string";

/// Name: Preferred line length<br>
/// Description: Max line length before formatting is applied.<br>
/// Short Name: pll<br>
/// Type: Integer
pub const PREFERRED_LINE_LENGTH: &str = "preferred-line-length";

/// Name: Banner<br>
/// Description: A message to display at the top of the decompiled file.<br>
/// Short Name: ban<br>
/// Type: String
pub const BANNER: &str = "banner";

/// Name: Error Message<br>
/// Description: Message to display when an error occurs in the decompiler.<br>
/// Short Name: erm<br>
/// Type: String
pub const ERROR_MESSAGE: &str = "error-message";

/// Name: Thread Count<br>
/// Description: How many threads to use to decompile.<br>
/// Dynamic Default Value: all available processors<br>
/// Short Name: thr<br>
/// Type: Integer
pub const THREADS: &str = "thread-count";

/// Name: Skip Extra Files<br>
/// Description: Skip copying non-class files from the input folder or file to the output<br>
/// Short Name: sef<br>
/// Type: Boolean
pub const SKIP_EXTRA_FILES: &str = "skip-extra-files";

/// Name: Warn about inconsistent inner attributes<br>
/// Description: Warn about inconsistent inner class attributes<br>
/// Short Name: win<br>
/// Type: Boolean
pub const WARN_INCONSISTENT_INNER_CLASSES: &str = "warn-inconsistent-inner-attributes";

/// Name: Dump Bytecode On Error<br>
/// Description: Put the bytecode in the method body when an error occurs.<br>
/// Short Name: dbe<br>
/// Type: Boolean
pub const DUMP_BYTECODE_ON_ERROR: &str = "dump-bytecode-on-error";

/// Name: Dump Exceptions On Error<br>
/// Description: Put the exception message in the method body or source file when an error occurs.<br>
/// Short Name: dee<br>
/// Type: Boolean
pub const DUMP_EXCEPTION_ON_ERROR: &str = "dump-exception-on-error";

/// Name: Decompiler Comments<br>
/// Description: Sometimes, odd behavior of the bytecode or unfixable problems occur. This enables or disables the adding of those to the decompiled output.<br>
/// Short Name: dec<br>
/// Type: Boolean
pub const DECOMPILER_COMMENTS: &str = "decompiler-comments";

/// Name: SourceFile comments<br>
/// Description: Add debug comments showing the class SourceFile attribute if present.<br>
/// Short Name: sfc<br>
/// Type: Boolean
pub const SOURCE_FILE_COMMENTS: &str = "sourcefile-comments";

/// Name: Decompile complex constant-dynamic expressions<br>
/// Description: Some constant-dynamic expressions can't be converted to a single Java expression with identical run-time behaviour. This decompiles them to a similar non-lazy expression, marked with a comment.<br>
/// Short Name: dcc<br>
/// Type: Boolean
pub const DECOMPILE_COMPLEX_CONDYS: &str = "decompile-complex-constant-dynamic";

/// Name: Force JSR inline<br>
/// Description: Forces the processing of JSR instructions even if the class files shouldn't contain it (Java 7+)<br>
/// Short Name: fji<br>
/// Type: Boolean
pub const FORCE_JSR_INLINE: &str = "force-jsr-inline";

/// Name: Dump Text Tokens<br>
/// Description: Dump Text Tokens on each class file<br>
/// Short Name: dtt<br>
/// Type: Boolean
pub const DUMP_TEXT_TOKENS: &str = "dump-text-tokens";

/// Name: Remove Imports<br>
/// Description: Remove import statements from the decompiled code<br>
/// Short Name: rim<br>
/// Type: Boolean
pub const REMOVE_IMPORTS: &str = "remove-imports";

/// Name: Mark Corresponding Synthetics<br>
/// Description: Mark lambdas and anonymous and local classes with their respective synthetic constructs<br>
/// Short Name: mcs<br>
/// Type: Boolean
pub const MARK_CORRESPONDING_SYNTHETICS: &str = "mark-corresponding-synthetics";

/// Name: Excluded Classes<br>
/// Description: Exclude classes from decompilation if their fully qualified names match the specified regular expression.<br>
/// Type: String
pub const EXCLUDED_CLASSES: &str = "excluded-classes";

/// Name: Validate inner classes names<br>
/// Description: Validates that the inner class name is correct (if it is separated using "\$" for example BaseClass$InnerClass). If not then inner class won't be processed.<br>
/// Type: String
pub const VALIDATE_INNER_CLASSES_NAMES: &str = "validate-inner-classes-names";
