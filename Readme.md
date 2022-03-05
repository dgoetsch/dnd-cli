## DND 5e Character CLI

This toy application emulates basic actions for interacting with a dnd character.  This means rolling dice, tracking inventory, and tracking hitpoints.

Rolls are represented via a template, and bonuses are granted based on rules.  There is a template file in the project to be used for a character - this is intented to be extensible to support several different kinds of characters; new abilities can be added as data without changing code, but some use cases may not be fully supported yet.

```bash
# install
## Load the configuration file
mkdir ~/.dnd-cli
cp characters/template.json ~/.dnd-cli/
## install binary on path
cargo install --path .
## bash completion
eval "$(dnd-cli completions bash)"

# navigate to directory where character is stored
cd characters/fluffy
# run commands!
dnd-cli character hit-points show
dnd-cli character roll skill deception
dnd-cli character inventory show
```