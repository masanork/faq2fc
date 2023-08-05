faq2fc: FAQ Markdown to FineTuning JSON
===

The "faq2fc" tool is used to convert FAQ Markdown to FineTuning JSON format. The default input file is specified through the command line argument, and the output is directed to the standard output (stdout) and also saved in a same_name_file.json in the same directory.
In the input file, the H1 tag, H4 and more tag and body text are ignored. H2 tags represent questions, and H3 tags represent answers. The 2nd occurrence of H2 and H3 tags are treated as the next prompt and its corresponding completion, respectively. This tool is designed for use in Azure OpenAI FineTuning.

The command-line program should support the following options:
Use the -v or --version option to display version information.
Non argument or Use the -h or --help option to display usage instructions.
1st argument to specify an input file.
Use the -o or --output option to specify an output file.

Input FAQ Markdown Format
---

```markdown
# This is FAQ of faq2fc tool

## What is faq2fc tool?

### faq2fc tool is convert FAQ Markdown to FineTuning JSON file for Azure OpenAI FineTuning use.

#### Comment out body text, H4 tag and more.

Comment out body text, H4 tag and more.

## next prompt

### next completion

```

Output FineTuning JSON Format
---

```json
{"prompt": "What is faq2fc tool?", "completion": "<faq2fc tool is convert FAQ Markdown to FineTuning JSON file for Azure OpenAI FineTuning use."}
{"prompt": "next prompt", "completion": "next completion"}
{"prompt": "<prompt text>", "completion": "<ideal generated text>"}
```
