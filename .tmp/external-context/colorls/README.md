---
source: GitHub Repository Analysis
library: colorls
package: colorls
topic: complete documentation
fetched: 2026-05-01T00:00:00Z
official_docs: https://github.com/athityakumar/colorls
---

# ColorLS Icon and Color System - Complete Documentation

## Overview

ColorLS is a Ruby gem that colorizes terminal `ls` output with file type icons. This document contains the complete icon mapping database extracted from the colorls source code.

## Icon Font Used

ColorLS uses **Nerd Fonts** and **Font Awesome** glyphs, which are Unicode characters in the **Private Use Area (PUA)**:
- **Nerd Fonts PUA**: U+E000 to U+F8FF (primary)
- **Font Awesome**: Standard Unicode positions (U+F000+ range)

To display these icons correctly, you **MUST** use a Nerd Font patched font in your terminal. Popular options:
- Hack Nerd Font
- Fira Code Nerd Font
- JetBrains Mono Nerd Font
- Meslo Nerd Font

## How Icons Are Rendered

1. **Icon Storage**: Icons are stored as Unicode strings in YAML files
2. **Loading**: The `yaml.rb` module loads YAML files from `lib/yaml/`
3. **Processing**: In `core.rb`, the method `fetch_string` processes icons:
   ```ruby
   logo = value.gsub(/\\u[\da-f]{4}/i) { |m| [m[-4..].to_i(16)].pack('U') }
   ```
   This converts `\uXXXX` escape sequences to actual Unicode characters.
4. **Display**: Icons are rendered as text next to filenames:
   ```ruby
   entry = @icons ? "#{out_encode(logo)}  #{out_encode(name)}" : out_encode(name).to_s
   ```

## File Icon Mapping (files.yaml)

The main file `files.yaml` maps file extensions and special filenames to icons.

### Complete Mapping Table

| Extension/Filename | Icon | Unicode Codepoint | Description |
|-------------------|------|-------------------|-------------|
| ai |  | U+E7B4 | Adobe Illustrator |
| android |  | U+E70E | Android |
| apple |  | U+F179 | Apple |
| audio |  | U+F001 | Audio file |
| avro |  | U+E60B | Avro |
| c |  | U+E61E | C |
| clj |  | U+E768 | Clojure |
| coffee |  | U+F0F4 | CoffeeScript |
| conf |  | U+E615 | Config file |
| cpp |  | U+E61D | C++ |
| css |  | U+E749 | CSS |
| d |  | U+E7AF | D |
| dart |  | U+E718 | Dart |
| db |  | U+F0C0 | Database |
| diff |  | U+F440 | Diff file |
| doc |  | U+F0C2 | Document |
| docker |  | U+F308 | Docker |
| ebook |  | U+E70B | Ebook |
| env |  | U+F462 | Environment file |
| epub |  | U+E70A | EPUB |
| erl |  | U+E7B1 | Erlang |
| file |  | U+F016 | Generic file |
| font |  | U+F031 | Font file |
| gform |  | U+F298 | Google Forms |
| git |  | U+F1D3 | Git |
| go |  | U+E626 | Go |
| gruntfile.js |  | U+E74C | Grunt |
| hs |  | U+E777 | Haskell |
| html |  | U+F13B | HTML5 |
| image |  | U+F0C5 | Image file |
| iml |  | U+E7B5 | IntelliJ |
| java |  | U+E204 | Java |
| js |  | U+E74E | JavaScript |
| json |  | U+E60B | JSON |
| jsx |  | U+E7BA | JSX |
| less |  | U+E758 | Less |
| log |  | U+F0C6 | Log file |
| lua |  | U+E620 | Lua |
| md |  | U+F48A | Markdown |
| mustache |  | U+E60F | Mustache |
| npmignore |  | U+E71E | npm |
| pdf |  | U+F1C1 | PDF |
| php |  | U+E73D | PHP |
| pl |  | U+E769 | Perl |
| ppt |  | U+E704 | PowerPoint |
| prql |  | U+E706 | PRQL |
| psd |  | U+E7B8 | Photoshop |
| py |  | U+E606 | Python |
| r |  | U+F25D | R |
| rb |  | U+E21E | Ruby |
| rdb |  | U+E76D | RDB |
| react | ﰆ | U+FB06 | React |
| rss |  | U+F09E | RSS |
| rubydoc |  | U+E73B | RubyDoc |
| sass |  | U+E603 | Sass |
| scala |  | U+E737 | Scala |
| shell |  | U+F489 | Shell |
| sqlite3 |  | U+E7C4 | SQLite |
| styl |  | U+E600 | Stylus |
| tex |  | U+E600 | TeX |
| ts |  | U+E628 | TypeScript |
| twig |  | U+E61C | Twig |
| txt |  | U+F01C | Text file |
| video |  | U+F03D | Video file |
| vim |  | U+E62B | Vim |
| vue | ﵂ | U+FD42 | Vue |
| windows |  | U+F17A | Windows |
| xls |  | U+F1C3 | Excel |
| xml |  | U+E619 | XML |
| yarn.lock |  | U+E718 | Yarn |
| yml |  | U+F481 | YAML |
| zip |  | U+F410 | Archive |

## File Extension Aliases (file_aliases.yaml)

Maps additional file extensions to the main categories above.

### Key Aliases

| Alias Extension | Maps To | Description |
|-----------------|---------|-------------|
| apk | android | Android package |
| gradle | android | Gradle build |
| ds_store | apple | macOS metadata |
| flac, m4a, mp3, ogg, opus, wav | audio | Audio formats |
| editorconfig | conf | Editor config |
| scss | css | Sass CSS |
| docx, gdoc, odt | doc | Document formats |
| dockerfile | docker | Dockerfile |
| mobi | ebook | Mobipocket ebook |
| eot, otf, ttf, woff, woff2 | font | Font files |
| gitconfig, gitignore, gitignore_global | git | Git config files |
| lhs | hs | Literate Haskell |
| avif, bmp, gif, heif, ico, jpeg, jpg, jxl, png, svg, tiff, webp | image | Image formats |
| jar | java | Java archive |
| mjs | js | ES Module |
| properties | json | Properties file |
| tsx | jsx | TypeScript JSX |
| license, markdown, mkd, rdoc, readme | md | Markdown files |
| gslides, odp, pptx | ppt | Presentation files |
| ipynb, pyc | py | Python files |
| rdata, rds | r | R data files |
| gemfile, gemspec, guardfile, lock, procfile, rakefile, rspec, ru | rb | Ruby files |
| erb, slim | rubydoc | Ruby documentation |
| bash, bash_history, bash_profile, bashrc, fish, sh, zsh, zshrc | shell | Shell scripts |
| stylus | styl | Stylus |
| cls | tex | TeX class |
| avi, flv, mkv, mov, mp4, ogv, webm | video | Video formats |
| bat, exe, ini | windows | Windows files |
| csv, gsheet, ods, xlsx | xls | Spreadsheet files |
| xul | xml | XML files |
| yaml | yml | YAML files |
| 7z, gz, rar, tar, tgz, xz | zip | Archive files |

## Folder Icon Mapping (folders.yaml)

| Folder Name | Icon | Unicode Codepoint | Description |
|-------------|------|-------------------|-------------|
| .atom |  | U+E765 | Atom editor folder |
| .git |  | U+F1D3 | Git folder |
| .github |  | U+F408 | GitHub folder |
| .rvm |  | U+E21E | RVM folder |
| .Trash |  | U+F1F8 | Trash folder |
| .vscode |  | U+E70C | VS Code folder |
| config |  | U+E5FC | Config folder |
| folder |  | U+F115 | Default folder |
| hidden |  | U+F023 | Hidden folder |
| lib |  | U+F121 | Library folder |
| node_modules |  | U+E718 | Node modules |

## Folder Aliases (folder_aliases.yaml)

| Alias Folder | Maps To |
|--------------|---------|
| bin | config |
| include | config |

## Color Schemes

### Dark Colors (for dark terminal backgrounds)

| Color Key | Color Name | Used For |
|-----------|------------|----------|
| unrecognized_file | gold | Files without icon mapping |
| recognized_file | yellow | Files with icon mapping |
| executable_file | lime | Executable files |
| dir | dodgerblue | Directories |
| dead_link | red | Broken symlinks |
| link | cyan | Symlinks |
| socket | green | Socket files |
| blockdev | green | Block devices |
| chardev | green | Character devices |
| hidden | burlywood | Hidden files |
| hidden_dir | slategray | Hidden directories |
| write | darkkhaki | Write permission |
| read | limegreen | Read permission |
| exec | red | Execute permission |
| no_access | indianred | No access |
| day_old | mediumspringgreen | Modified within a day |
| hour_old | lime | Modified within an hour |
| no_modifier | seagreen | Older modifications |
| file_large | orange | Large files (≥512MB) |
| file_medium | gold | Medium files (≥128MB) |
| file_small | peachpuff | Small files |
| report | white | Report text |
| user | moccasin | User name |
| tree | cyan | Tree structure |
| empty | yellow | Empty directory message |
| error | red | Error messages |
| normal | darkkhaki | Normal text |
| inode | moccasin | Inode numbers |
| addition | chartreuse | Git added files |
| modification | darkkhaki | Git modified files |
| deletion | darkred | Git deleted files |
| untracked | darkorange | Git untracked files |
| unchanged | forestgreen | Git unchanged files |

### Light Colors (for light terminal backgrounds)

Same keys as dark colors, but with darker color names suitable for light backgrounds.

## How to Use This Data

### Ruby Example (how colorls does it)

```ruby
require 'yaml'

# Load icon mappings
files = YAML.load_file('files.yaml')
file_aliases = YAML.load_file('file_aliases.yaml')

# Get icon for a file
def get_icon(filename)
  ext = File.extname(filename).delete_prefix('.')
  key = ext.to_sym
  
  # Check main mapping first
  return files[key] if files.key?(key)
  
  # Check aliases
  if file_aliases.key?(key)
    return files[file_aliases[key].to_sym]
  end
  
  # Default
  return files[:file]
end
```

### Python Example

```python
import yaml

# Load icon mappings
with open('files.yaml', 'r') as f:
    files = yaml.safe_load(f)

with open('file_aliases.yaml', 'r') as f:
    file_aliases = yaml.safe_load(f)

def get_icon(filename):
    ext = os.path.splitext(filename)[1].lstrip('.')
    
    # Check main mapping first
    if ext in files:
        return files[ext]
    
    # Check aliases
    if ext in file_aliases:
        alias_key = file_aliases[ext]
        return files.get(alias_key)
    
    # Default
    return files.get('file', '')
```

## Important Notes

1. **Nerd Fonts Required**: The icons are Nerd Font glyphs. You need a patched font to display them.
2. **Unicode PUA**: The icons use Unicode Private Use Area characters (U+E000-U+F8FF).
3. **UTF-8 Encoding**: The files are UTF-8 encoded. Ensure your terminal supports UTF-8.
4. **Fallback**: If an icon isn't found, colorls uses the generic "file" icon ().
5. **Aliases**: File aliases allow multiple extensions to map to the same icon category.

## Source Files

- Main repo: https://github.com/athityakumar/colorls
- Icon definitions: `lib/yaml/files.yaml`
- File aliases: `lib/yaml/file_aliases.yaml`
- Folder icons: `lib/yaml/folders.yaml`
- Folder aliases: `lib/yaml/folder_aliases.yaml`
- Core logic: `lib/colorls/core.rb`
- YAML loader: `lib/colorls/yaml.rb`
