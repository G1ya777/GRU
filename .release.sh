#!/bin/bash

echo "What's the new version:"
read new_version

file1=./Cargo.toml
file2=./AUR/PKGBUILD




echo "is the version $new_version correct ?"

read confirm

if [[ $confirm != "y" && $confirm != "Y" ]]; then
    echo "Version change aborted. Exiting script."
    exit 1
fi

sed -i "0,/version = \"[^\"]*\"/s//version = \"$new_version\"/" "$file1"
sed -i 's/^pkgver=.*/pkgver='"$new_version"'/' "$file2"

echo "Version updated to $new_version in $file1 and $file2."

program_help_output=$(just help 2>&1)
program_help_output=$(echo "$program_help_output" | sed '1,2d')


printf "<!-- rename-gru --help output -->\n\`\`\`\n%s\n\`\`\`\n<!-- end of rename-gru --help command output -->
" "$program_help_output" > help_output.tmp

readme=readme.md


start_comment_line_number=$(grep -n "<!-- rename-gru --help output -->" "$readme" | cut -d ":" -f 1)
end_comment_line_number=$(grep -n "<!-- end of rename-gru --help command output -->" "$readme" | cut -d ":" -f 1) 



sed -i "${start_comment_line_number},${end_comment_line_number}d" "$readme"


((start_comment_line_number -= 2))


awk -v line_number="$start_comment_line_number" 'NR == line_number + 1 {print; system("cat help_output.tmp"); next} 1' "$readme" > tmpfile && mv tmpfile "$readme"

rm help_output.tmp

echo "README updated with rename-gru --help output."


echo "to push write PUSH: "

read confirm

if [[ $confirm != "PUSH" && $confirm != "Y" ]]; then
    echo "Push aborted. Exiting script."
    exit 1
fi

git add .
git commit -m "v$new_version"
git push


cd AUR
makepkg --printsrcinfo > .SRCINFO
git add .
git commit -m "v$new_version"
git push

