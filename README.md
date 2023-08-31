# Clink

## Console autotyper

Usecase: When you have a long paddword and dont want to type it into a IPMI console

Usage:

Clink is a very simple application it can either receive the string as a argument or have the contents piped into it.

Example:
`clink '<your_super_secret_string>'`

or

`cat file.txt | clink`

Then all you have to do is click your mouse. The string will be typed after a 2 second wait.

