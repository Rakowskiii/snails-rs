
num_of_labs := `ls labs | wc -l`

@_default: 
    just --list

_get_lab_number $lab_number:
    #!/bin/bash
    packages_str=$(cargo metadata --format-version 1 --no-deps | jq -r '.packages[] .name')
    lab_name=$(echo "$packages_str" | awk -v n="$((lab_number+1))" 'NR == n')
    echo $lab_name

test lab_number:
    #!/bin/bash
    lab_name=$(just _get_lab_number {{lab_number}}) 
    cargo test --package $lab_name test_proper_flow -- --nocapture

hack $lab_number:
    #!/bin/bash
    lab_name=$(just _get_lab_number {{lab_number}})
    cargo test --package $lab_name hack -- --nocapture
