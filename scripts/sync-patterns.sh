#!/usr/bin/env bash
# Generate Android Java and PWA JS pattern arrays from patterns.csv.
# Rust platforms use build.rs instead. Run this after editing patterns.csv.
set -euo pipefail

CSV="$(dirname "$0")/../patterns.csv"
JAVA="$(dirname "$0")/../android/app/src/main/java/org/cochranblock/callshield/IntentClassifier.java"
HTML="$(dirname "$0")/../web/index.html"

if [ ! -f "$CSV" ]; then echo "patterns.csv not found"; exit 1; fi

# --- Build Java arrays ---
spam_java=""
legit_java=""
while IFS=, read -r cat pat wt; do
    [ "$cat" = "category" ] && continue
    [ -z "$cat" ] && continue
    entry="        {\"$pat\", \"$wt\"},"
    if [ "$cat" = "spam" ]; then
        spam_java+="$entry"$'\n'
    else
        legit_java+="$entry"$'\n'
    fi
done < "$CSV"

# Remove trailing comma from last entry
spam_java=$(echo "$spam_java" | sed '$ s/,$//')
legit_java=$(echo "$legit_java" | sed '$ s/,$//')

# Replace Java SPAM array
python3 -c "
import re, sys
with open('$JAVA') as f: src = f.read()
spam = '''$spam_java'''
legit = '''$legit_java'''
src = re.sub(
    r'(private static final String\[\]\[\] SPAM = \{)\n.*?(\n    \};)',
    r'\1\n' + spam.rstrip() + r'\2',
    src, flags=re.DOTALL)
src = re.sub(
    r'(private static final String\[\]\[\] LEGIT = \{)\n.*?(\n    \};)',
    r'\1\n' + legit.rstrip() + r'\2',
    src, flags=re.DOTALL)
with open('$JAVA', 'w') as f: f.write(src)
"

# --- Build JS arrays ---
spam_js=""
legit_js=""
while IFS=, read -r cat pat wt; do
    [ "$cat" = "category" ] && continue
    [ -z "$cat" ] && continue
    entry="        [\"$pat\", $wt],"
    if [ "$cat" = "spam" ]; then
        spam_js+="$entry "
    else
        legit_js+="$entry "
    fi
done < "$CSV"

python3 -c "
import re
with open('$HTML') as f: src = f.read()
spam = '''$spam_js'''.strip().rstrip(',')
legit = '''$legit_js'''.strip().rstrip(',')
# Reformat: 2 entries per line
def fmt(entries):
    items = [e.strip() for e in entries.split('],') if e.strip()]
    items = [e + ']' if not e.endswith(']') else e for e in items]
    lines = []
    for i in range(0, len(items), 2):
        chunk = ', '.join(items[i:i+2])
        lines.append('        ' + chunk + ',')
    lines[-1] = lines[-1].rstrip(',')  # no trailing comma on last
    return '\n'.join(lines)

src = re.sub(
    r'(const SPAM = \[)\n.*?(\n    \];)',
    r'\1\n' + fmt(spam) + r'\2',
    src, flags=re.DOTALL)
src = re.sub(
    r'(const LEGIT = \[)\n.*?(\n    \];)',
    r'\1\n' + fmt(legit) + r'\2',
    src, flags=re.DOTALL)
with open('$HTML', 'w') as f: f.write(src)
"

echo "Synced patterns.csv -> IntentClassifier.java + index.html"
