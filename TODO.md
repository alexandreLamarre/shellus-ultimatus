# TODO

## Shell features

- [ ] shell syntax
  - Shell operation
  - Quoting
  - Comments

```
3.1.1 Shell Operation
The following is a brief description of the shell’s operation when it reads and executes a command. Basically, the shell does the following:

Reads its input from a file (see Shell Scripts), from a string supplied as an argument to the -c invocation option (see Invoking Bash), or from the user’s terminal.
Breaks the input into words and operators, obeying the quoting rules described in Quoting. These tokens are separated by metacharacters. Alias expansion is performed by this step (see Aliases).
Parses the tokens into simple and compound commands (see Shell Commands).
Performs the various shell expansions (see Shell Expansions), breaking the expanded tokens into lists of filenames (see Filename Expansion) and commands and arguments.
Performs any necessary redirections (see Redirections) and removes the redirection operators and their operands from the argument list.
Executes the command (see Executing Commands).
Optionally waits for the command to complete and collects its exit status (see Exit Status).
```

```
• Escape Character	  	How to remove the special meaning from a single character.
• Single Quotes	  	How to inhibit all interpretation of a sequence of characters.
• Double Quotes	  	How to suppress most of the interpretation of a sequence of characters.
• ANSI-C Quoting	  	How to expand ANSI-C sequences in quoted strings.
• Locale Translation	  	How to translate strings into different languages.
Quoting is used to remove the special meaning of certain characters or words to the shell. Quoting can be used to disable special treatment for special characters, to prevent reserved words from being recognized as such, and to prevent parameter expansion.

Each of the shell metacharacters (see Definitions) has special meaning to the shell and must be quoted if it is to represent itself. When the command history expansion facilities are being used (see History Interaction), the history expansion character, usually ‘!’, must be quoted to prevent history expansion. See Bash History Facilities, for more details concerning history expansion.

There are three quoting mechanisms: the escape character, single quotes, and double quotes.
```

```
A non-quoted backslash ‘\’ is the Bash escape character. It preserves the literal value of the next character that follows, with the exception of newline. If a \newline pair appears, and the backslash itself is not quoted, the \newline is treated as a line continuation (that is, it is removed from the input stream and effectively ignored).
```

```
Enclosing characters in single quotes (‘'’) preserves the literal value of each character within the quotes. A single quote may not occur between single quotes, even when preceded by a backslash.
```

```
Enclosing characters in double quotes (‘"’) preserves the literal value of all characters within the quotes, with the exception of ‘$’, ‘`’, ‘\’, and, when history expansion is enabled, ‘!’. When the shell is in POSIX mode (see Bash POSIX Mode), the ‘!’ has no special meaning within double quotes, even when history expansion is enabled. The characters ‘$’ and ‘`’ retain their special meaning within double quotes (see Shell Expansions). The backslash retains its special meaning only when followed by one of the following characters: ‘$’, ‘`’, ‘"’, ‘\’, or newline. Within double quotes, backslashes that are followed by one of these characters are removed. Backslashes preceding characters without a special meaning are left unmodified. A double quote may be quoted within double quotes by preceding it with a backslash. If enabled, history expansion will be performed unless an ‘!’ appearing in double quotes is escaped using a backslash. The backslash preceding the ‘!’ is not removed.

The special parameters ‘*’ and ‘@’ have special meaning when in double quotes (see Shell Parameter Expansion).
```

```
Words of the form $'string' are treated specially. The word expands to string, with backslash-escaped characters replaced as specified by the ANSI C standard. Backslash escape sequences, if present, are decoded as follows:

\a
alert (bell)

\b
backspace

\e
\E
an escape character (not ANSI C)

\f
form feed

\n
newline

\r
carriage return

\t
horizontal tab

\v
vertical tab

\\
backslash

\'
single quote

\"
double quote

\?
question mark

\nnn
the eight-bit character whose value is the octal value nnn (one to three octal digits)

\xHH
the eight-bit character whose value is the hexadecimal value HH (one or two hex digits)

\uHHHH
the Unicode (ISO/IEC 10646) character whose value is the hexadecimal value HHHH (one to four hex digits)

\UHHHHHHHH
the Unicode (ISO/IEC 10646) character whose value is the hexadecimal value HHHHHHHH (one to eight hex digits)

\cx
a control-x character

The expanded result is single-quoted, as if the dollar sign had not been present.
```

```
3.1.2.5 Locale-Specific Translation
A double-quoted string preceded by a dollar sign (‘$’) will cause the string to be translated according to the current locale. The gettext infrastructure performs the message catalog lookup and translation, using the LC_MESSAGES and TEXTDOMAIN shell variables, as explained below. See the gettext documentation for additional details. If the current locale is C or POSIX, or if there are no translations available, the dollar sign is ignored. If the string is translated and replaced, the replacement is double-quoted.

Some systems use the message catalog selected by the LC_MESSAGES shell variable. Others create the name of the message catalog from the value of the TEXTDOMAIN shell variable, possibly adding a suffix of ‘.mo’. If you use the TEXTDOMAIN variable, you may need to set the TEXTDOMAINDIR variable to the location of the message catalog files. Still others use both variables in this fashion: TEXTDOMAINDIR/LC_MESSAGES/LC_MESSAGES/TEXTDOMAIN.mo.
```

- [ ] shell commands

  - Reserved words
  - Simple commands
  - Pipelines
  - Lists
  - Compound Commands
  - Coprocess
  - GNU Parallel

  ```
  3.2.1 Reserved Words
  Reserved words are words that have special meaning to the shell. They are used to begin and end the shell’s compound commands.
  ```

The following words are recognized as reserved when unquoted and the first word of a command (see below for exceptions):

if then elif else fi time
for in until while do done
case esac coproc select function
{ } [[]] !
in is recognized as a reserved word if it is the third word of a case or select command. in and do are recognized as reserved words if they are the third word in a for command.

```

```

3.2.2 Simple Commands
A simple command is the kind of command encountered most often. It’s just a sequence of words separated by blanks, terminated by one of the shell’s control operators (see Definitions). The first word generally specifies a command to be executed, with the rest of the words being that command’s arguments.

The return status (see Exit Status) of a simple command is its exit status as provided by the POSIX 1003.1 waitpid function, or 128+n if the command was terminated by signal n.

```

```

3.2.3 Pipelines
A pipeline is a sequence of one or more commands separated by one of the control operators ‘|’ or ‘|&’.

The format for a pipeline is

[time [-p]] [!] command1 [ | or |& command2 ] …
The output of each command in the pipeline is connected via a pipe to the input of the next command. That is, each command reads the previous command’s output. This connection is performed before any redirections specified by the command.

If ‘|&’ is used, command1’s standard error, in addition to its standard output, is connected to command2’s standard input through the pipe; it is shorthand for 2>&1 |. This implicit redirection of the standard error to the standard output is performed after any redirections specified by the command.

The reserved word time causes timing statistics to be printed for the pipeline once it finishes. The statistics currently consist of elapsed (wall-clock) time and user and system time consumed by the command’s execution. The -p option changes the output format to that specified by POSIX. When the shell is in POSIX mode (see Bash POSIX Mode), it does not recognize time as a reserved word if the next token begins with a ‘-’. The TIMEFORMAT variable may be set to a format string that specifies how the timing information should be displayed. See Bash Variables, for a description of the available formats. The use of time as a reserved word permits the timing of shell builtins, shell functions, and pipelines. An external time command cannot time these easily.

When the shell is in POSIX mode (see Bash POSIX Mode), time may be followed by a newline. In this case, the shell displays the total user and system time consumed by the shell and its children. The TIMEFORMAT variable may be used to specify the format of the time information.

If the pipeline is not executed asynchronously (see Lists), the shell waits for all commands in the pipeline to complete.

Each command in a pipeline is executed in its own subshell, which is a separate process (see Command Execution Environment). If the lastpipe option is enabled using the shopt builtin (see The Shopt Builtin), the last element of a pipeline may be run by the shell process.

The exit status of a pipeline is the exit status of the last command in the pipeline, unless the pipefail option is enabled (see The Set Builtin). If pipefail is enabled, the pipeline’s return status is the value of the last (rightmost) command to exit with a non-zero status, or zero if all commands exit successfully. If the reserved word ‘!’ precedes the pipeline, the exit status is the logical negation of the exit status as described above. The shell waits for all commands in the pipeline to terminate before returning a value.

```

```

3.2.4 Lists of Commands
A list is a sequence of one or more pipelines separated by one of the operators ‘;’, ‘&’, ‘&&’, or ‘||’, and optionally terminated by one of ‘;’, ‘&’, or a newline.

Of these list operators, ‘&&’ and ‘||’ have equal precedence, followed by ‘;’ and ‘&’, which have equal precedence.

A sequence of one or more newlines may appear in a list to delimit commands, equivalent to a semicolon.

If a command is terminated by the control operator ‘&’, the shell executes the command asynchronously in a subshell. This is known as executing the command in the background, and these are referred to as asynchronous commands. The shell does not wait for the command to finish, and the return status is 0 (true). When job control is not active (see Job Control), the standard input for asynchronous commands, in the absence of any explicit redirections, is redirected from /dev/null.

Commands separated by a ‘;’ are executed sequentially; the shell waits for each command to terminate in turn. The return status is the exit status of the last command executed.

AND and OR lists are sequences of one or more pipelines separated by the control operators ‘&&’ and ‘||’, respectively. AND and OR lists are executed with left associativity.

An AND list has the form

command1 && command2
command2 is executed if, and only if, command1 returns an exit status of zero (success).

An OR list has the form

command1 || command2
command2 is executed if, and only if, command1 returns a non-zero exit status.

The return status of AND and OR lists is the exit status of the last command executed in the list.

```

```

3.2.5 Compound Commands
• Looping Constructs Shell commands for iterative action.
• Conditional Constructs Shell commands for conditional execution.
• Command Grouping Ways to group commands.
Compound commands are the shell programming language constructs. Each construct begins with a reserved word or control operator and is terminated by a corresponding reserved word or operator. Any redirections (see Redirections) associated with a compound command apply to all commands within that compound command unless explicitly overridden.

In most cases a list of commands in a compound command’s description may be separated from the rest of the command by one or more newlines, and may be followed by a newline in place of a semicolon.

Bash provides looping constructs, conditional commands, and mechanisms to group commands and execute them as a unit.

```

```

3.2.6 Coprocesses
A coprocess is a shell command preceded by the coproc reserved word. A coprocess is executed asynchronously in a subshell, as if the command had been terminated with the ‘&’ control operator, with a two-way pipe established between the executing shell and the coprocess.

The format for a coprocess is:

coproc [NAME] command [redirections]
This creates a coprocess named NAME. If NAME is not supplied, the default name is COPROC. NAME must not be supplied if command is a simple command (see Simple Commands); otherwise, it is interpreted as the first word of the simple command.

When the coprocess is executed, the shell creates an array variable (see Arrays) named NAME in the context of the executing shell. The standard output of command is connected via a pipe to a file descriptor in the executing shell, and that file descriptor is assigned to NAME[0]. The standard input of command is connected via a pipe to a file descriptor in the executing shell, and that file descriptor is assigned to NAME[1]. This pipe is established before any redirections specified by the command (see Redirections). The file descriptors can be utilized as arguments to shell commands and redirections using standard word expansions. Other than those created to execute command and process substitutions, the file descriptors are not available in subshells.

The process ID of the shell spawned to execute the coprocess is available as the value of the variable NAME_PID. The wait builtin command may be used to wait for the coprocess to terminate.

Since the coprocess is created as an asynchronous command, the coproc command always returns success. The return status of a coprocess is the exit status of command.

```

```

3.2.7 GNU Parallel
There are ways to run commands in parallel that are not built into Bash. GNU Parallel is a tool to do just that.

GNU Parallel, as its name suggests, can be used to build and run commands in parallel. You may run the same command with different arguments, whether they are filenames, usernames, hostnames, or lines read from files. GNU Parallel provides shorthand references to many of the most common operations (input lines, various portions of the input line, different ways to specify the input source, and so on). Parallel can replace xargs or feed commands from its input sources to several different instances of Bash.

For a complete description, refer to the GNU Parallel documentation. A few examples should provide a brief introduction to its use.

For example, it is easy to replace xargs to gzip all html files in the current directory and its subdirectories:

find . -type f -name '\*.html' -print | parallel gzip
If you need to protect special characters such as newlines in file names, use find’s -print0 option and parallel’s -0 option.

You can use Parallel to move files from the current directory when the number of files is too large to process with one mv invocation:

printf '%s\n' _ | parallel mv {} destdir
As you can see, the {} is replaced with each line read from standard input. While using ls will work in most instances, it is not sufficient to deal with all filenames. printf is a shell builtin, and therefore is not subject to the kernel’s limit on the number of arguments to a program, so you can use ‘_’ (but see below about the dotglob shell option). If you need to accommodate special characters in filenames, you can use

printf '%s\0' \* | parallel -0 mv {} destdir
as alluded to above.

This will run as many mv commands as there are files in the current directory. You can emulate a parallel xargs by adding the -X option:

printf '%s\0' \* | parallel -0 -X mv {} destdir
(You may have to modify the pattern if you have the dotglob option enabled.)

GNU Parallel can replace certain common idioms that operate on lines read from a file (in this case, filenames listed one per line):

    while IFS= read -r x; do
    	do-something1 "$x" "config-$x"
    	do-something2 < "$x"
    done < file | process-output

with a more compact syntax reminiscent of lambdas:

cat list | parallel "do-something1 {} config-{} ; do-something2 < {}" |
process-output
Parallel provides a built-in mechanism to remove filename extensions, which lends itself to batch file transformations or renaming:

ls \*.gz | parallel -j+0 "zcat {} | bzip2 >{.}.bz2 && rm {}"
This will recompress all files in the current directory with names ending in .gz using bzip2, running one job per CPU (-j+0) in parallel. (We use ls for brevity here; using find as above is more robust in the face of filenames containing unexpected characters.) Parallel can take arguments from the command line; the above can also be written as

parallel "zcat {} | bzip2 >{.}.bz2 && rm {}" ::: \*.gz
If a command generates output, you may want to preserve the input order in the output. For instance, the following command

{
echo foss.org.my ;
echo debian.org ;
echo freenetproject.org ;
} | parallel traceroute
will display as output the traceroute invocation that finishes first. Adding the -k option

{
echo foss.org.my ;
echo debian.org ;
echo freenetproject.org ;
} | parallel -k traceroute
will ensure that the output of traceroute foss.org.my is displayed first.

Finally, Parallel can be used to run a sequence of shell commands in parallel, similar to ‘cat file | bash’. It is not uncommon to take a list of filenames, create a series of shell commands to operate on them, and feed that list of commands to a shell. Parallel can speed this up. Assuming that file contains a list of shell commands, one per line,

parallel -j 10 < file
will evaluate the commands using the shell (since no explicit command is supplied as an argument), in blocks of ten shell jobs at a time.

```
- [ ] shell functions
```

3.3 Shell Functions
Shell functions are a way to group commands for later execution using a single name for the group. They are executed just like a "regular" command. When the name of a shell function is used as a simple command name, the list of commands associated with that function name is executed. Shell functions are executed in the current shell context; no new process is created to interpret them.

Functions are declared using this syntax:

fname () compound-command [ redirections ]
or

function fname [()] compound-command [ redirections ]
This defines a shell function named fname. The reserved word function is optional. If the function reserved word is supplied, the parentheses are optional. The body of the function is the compound command compound-command (see Compound Commands). That command is usually a list enclosed between { and }, but may be any compound command listed above, with one exception: If the function reserved word is used, but the parentheses are not supplied, the braces are required. compound-command is executed whenever fname is specified as the name of a command. When the shell is in POSIX mode (see Bash POSIX Mode), fname must be a valid shell name and may not be the same as one of the special builtins (see Special Builtins). In default mode, a function name can be any unquoted shell word that does not contain ‘$’. Any redirections (see Redirections) associated with the shell function are performed when the function is executed. A function definition may be deleted using the -f option to the unset builtin (see Bourne Shell Builtins).

The exit status of a function definition is zero unless a syntax error occurs or a readonly function with the same name already exists. When executed, the exit status of a function is the exit status of the last command executed in the body.

Note that for historical reasons, in the most common usage the curly braces that surround the body of the function must be separated from the body by blanks or newlines. This is because the braces are reserved words and are only recognized as such when they are separated from the command list by whitespace or another shell metacharacter. Also, when using the braces, the list must be terminated by a semicolon, a ‘&’, or a newline.

When a function is executed, the arguments to the function become the positional parameters during its execution (see Positional Parameters). The special parameter ‘#’ that expands to the number of positional parameters is updated to reflect the change. Special parameter 0 is unchanged. The first element of the FUNCNAME variable is set to the name of the function while the function is executing.

All other aspects of the shell execution environment are identical between a function and its caller with these exceptions: the DEBUG and RETURN traps are not inherited unless the function has been given the trace attribute using the declare builtin or the -o functrace option has been enabled with the set builtin, (in which case all functions inherit the DEBUG and RETURN traps), and the ERR trap is not inherited unless the -o errtrace shell option has been enabled. See Bourne Shell Builtins, for the description of the trap builtin.

The FUNCNEST variable, if set to a numeric value greater than 0, defines a maximum function nesting level. Function invocations that exceed the limit cause the entire command to abort.

If the builtin command return is executed in a function, the function completes and execution resumes with the next command after the function call. Any command associated with the RETURN trap is executed before execution resumes. When a function completes, the values of the positional parameters and the special parameter ‘#’ are restored to the values they had prior to the function’s execution. If a numeric argument is given to return, that is the function’s return status; otherwise the function’s return status is the exit status of the last command executed before the return.

Variables local to the function may be declared with the local builtin. These variables are visible only to the function and the commands it invokes. This is particularly important when a shell function calls other functions.

Local variables "shadow" variables with the same name declared at previous scopes. For instance, a local variable declared in a function hides a global variable of the same name: references and assignments refer to the local variable, leaving the global variable unmodified. When the function returns, the global variable is once again visible.

The shell uses dynamic scoping to control a variable’s visibility within functions. With dynamic scoping, visible variables and their values are a result of the sequence of function calls that caused execution to reach the current function. The value of a variable that a function sees depends on its value within its caller, if any, whether that caller is the "global" scope or another shell function. This is also the value that a local variable declaration "shadows", and the value that is restored when the function returns.

For example, if a variable var is declared as local in function func1, and func1 calls another function func2, references to var made from within func2 will resolve to the local variable var from func1, shadowing any global variable named var.

The following script demonstrates this behavior. When executed, the script displays

In func2, var = func1 local
func1()
{
local var='func1 local'
func2
}

func2()
{
echo "In func2, var = $var"
}

var=global
func1
The unset builtin also acts using the same dynamic scope: if a variable is local to the current scope, unset will unset it; otherwise the unset will refer to the variable found in any calling scope as described above. If a variable at the current local scope is unset, it will remain so until it is reset in that scope or until the function returns. Once the function returns, any instance of the variable at a previous scope will become visible. If the unset acts on a variable at a previous scope, any instance of a variable with that name that had been shadowed will become visible.

Function names and definitions may be listed with the -f option to the declare (typeset) builtin command (see Bash Builtins). The -F option to declare or typeset will list the function names only (and optionally the source file and line number, if the extdebug shell option is enabled). Functions may be exported so that subshells automatically have them defined with the -f option to the export builtin (see Bourne Shell Builtins).

Functions may be recursive. The FUNCNEST variable may be used to limit the depth of the function call stack and restrict the number of function invocations. By default, no limit is placed on the number of recursive calls.

```

- [ ] shell parameters
- [ ] shell expansions
- [ ] redirections
- [ ] executing commands
- [] shell scripts

## Frontend
```
