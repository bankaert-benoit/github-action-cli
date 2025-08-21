# Github action CLI

CLI to help list, check the systax and test locally github actions workflows.

## Roadmap

* list - List all the workflow
    * Add boilerplate option to list the boilerplate template workflow
* check - Check the syntax of the workflow in the given directory
* run - Run locally the workflow
* generate - Generate a workflow
    * Allow use of boilerplate for quick generate

## Commands

### List

List all the workflow, it will look for a .github folder and print print the name of the workflow files
If no .github folder is found, it will return an error

```
gac list
```

