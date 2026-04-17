## Configuration File Format

Options may be specified in a configuration file.

The format is similar to the configuration file format used by
[curl](https://everything.curl.dev/cmdline/configfile.html).


### Parsing Rules

- Options must begin with --.
- Each non-empty line represents a single argument after trimming leading and trailing whitespace.
- Lines that begin with `#` (optionally preceded by whitespace) are treated as comments and ignored.
- Empty lines are ignored.
- No shell parsing (e.g., variable expansion or escaping) is performed.


### Option value

- An option value must be provided on the same line as the option. It is separated by either one or more spaces or by `=`.
- Values containing spaces or newlines must be enclosed in double quotes (").
- Double-quoted values may span multiple lines; newlines are preserved. A quoted value ends at the next double quote (")


### Empty Values

The following forms are equivalent and represent an empty value:

--option=
--option=""


### Errors

The following conditions must result in an error:

- Unknown option
- An option that requires a value is not provided one
- Unclosed double quotes
- Unquoted values containing spaces or newlines
- characters after the closing quotes




### Example

```bash
$ cat $HOME/.config/hurl/config

# Standalone flag
--test

# Provide value after an equal
--header=foo:bar

# Provide value after a space
--variable user=bob

# Use unnecessary quotes
--retry="2"

# Use space in value
--user-agent="Mozilla/5.0 A"

# Use multiple line value
--variable "lines=line1
line2
line3"

# Use empty value
--user-agent=

# Use = value 
--user-agent==
--user-agent =
--user-agent="="


```

