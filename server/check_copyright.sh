#!/bin/sh
EXIT=0
for file in $(find src -name "*.rs"); do
    if ! grep -E -q "Copyright \(c\) 20[0-9]{2} PlaatSoft" "$file"; then
        echo "Bad copyright header in: $file"
        EXIT=1
    fi
done
exit $EXIT
