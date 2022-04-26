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
- [ ] shell functions
- [ ] shell parameters
- [ ] shell expansions
- [ ] redirections
- [ ] executing commands
- [] shell scripts

## Frontend
