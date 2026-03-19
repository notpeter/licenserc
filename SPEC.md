# `.licenserc.yaml` Specification

This document describes the `.licenserc.yaml` configuration file format, based on [Apache SkyWalking Eyes](https://github.com/apache/skywalking-eyes).

## Top-Level Structure

The configuration file has two main sections:

```yaml
header: # License header checking/fixing for source files
dependency: # Dependency license checking
```

---

## Header Section

The `header` section configures license header checking and insertion for source code files.

### V1 Format (Single Header)

```yaml
header:
  license:
    spdx-id: Apache-2.0
    copyright-owner: Apache Software Foundation
  paths:
    - "**"
  paths-ignore:
    - "**/*.md"
```

### V2 Format (Multiple Headers)

For multi-module projects with different licenses per path:

```yaml
header:
  - license:
      spdx-id: Apache-2.0
      copyright-owner: "Company A"
    paths:
      - "module-a/**"
  - license:
      spdx-id: MIT
      copyright-owner: "Company B"
    paths:
      - "module-b/**"
```

The tool tries V2 format first, then falls back to V1.

---

### `header.license`

Defines the license to check for and insert.

| Field             | Type   | Required | Default      | Description                                                                                             |
| ----------------- | ------ | -------- | ------------ | ------------------------------------------------------------------------------------------------------- |
| `spdx-id`         | string | No       | —            | SPDX license identifier (e.g. `Apache-2.0`, `MIT`). Used to select a built-in header template.          |
| `copyright-owner` | string | No       | —            | Copyright holder name. Replaces `[owner]` in SPDX templates.                                            |
| `copyright-year`  | string | No       | Current year | Copyright year or range (e.g. `"2024"`, `"2020-2024"`). Replaces `[year]` in templates.                 |
| `software-name`   | string | No       | —            | Project name. Replaces `[software-name]` in templates.                                                  |
| `content`         | string | No       | —            | Full license header text. Takes precedence over `spdx-id` when both are specified.                      |
| `pattern`         | string | No       | —            | Regex pattern to match existing license headers. Enables flexible validation of varying header formats. |

**Supported SPDX IDs** (with built-in header templates):

- `Apache-2.0`
- `Apache-2.0-EF` (Enterprise Friendly variant)
- `MIT`
- `GPL-3.0-or-later`
- `AGPL-3.0-only`
- `AGPL-3.0-or-later`
- `EPL-2.0`
- `MPL-2.0`
- `MulanPSL-2.0`

**Special case:** When `spdx-id` is `Apache-2.0` and `copyright-owner` matches `Apache Software Foundation` or `ASF` (case-insensitive), a dedicated ASF header template is used instead of the standard one.

---

### `header.paths`

List of glob patterns specifying which files to check. Supports doublestar (`**`) syntax.

- **Default:** `['**']` (all files)

```yaml
paths:
  - "**"
  - "src/**/*.go"
```

### `header.paths-ignore`

List of glob patterns specifying which files to exclude from checking. `.git` directory contents and `.gitignore` entries are automatically ignored.

```yaml
paths-ignore:
  - "dist"
  - "licenses"
  - "**/*.md"
  - "**/testdata/**"
  - "**/go.mod"
  - "**/go.sum"
  - "LICENSE"
  - "NOTICE"
```

---

### `header.comment`

Controls PR commenting behavior (for CI/GitHub Actions integration).

| Value        | Description                                 |
| ------------ | ------------------------------------------- |
| `on-failure` | Comment on PR only when license check fails |
| `always`     | Always comment on PR                        |
| `never`      | Never comment on PR                         |

---

### `header.license-location-threshold`

- **Type:** integer
- **Default:** `80`

Maximum number of characters from the start of a file where the license header can be located. A header found beyond this threshold is treated as missing.

---

### `header.language`

Override or customize comment styles for specific languages. Each entry is keyed by language name and supports:

| Field              | Type            | Description                                                        |
| ------------------ | --------------- | ------------------------------------------------------------------ |
| `extensions`       | list of strings | File extensions that trigger this language config (e.g. `[".go"]`) |
| `filenames`        | list of strings | Specific filenames to match (e.g. `["Makefile"]`)                  |
| `comment_style_id` | string          | Reference to a comment style (see Comment Styles below)            |

```yaml
language:
  Go:
    extensions:
      - ".go"
    comment_style_id: DoubleSlash
  YAML:
    extensions:
      - ".yaml"
      - ".yml"
    comment_style_id: Hashtag
```

---

## Comment Styles

Comment styles define how license headers are wrapped in language-appropriate comment syntax.

Each style has:

| Field           | Type   | Description                                                                |
| --------------- | ------ | -------------------------------------------------------------------------- |
| `id`            | string | Unique identifier                                                          |
| `start`         | string | Opening comment delimiter                                                  |
| `middle`        | string | Line prefix for interior lines (`~` means no prefix)                       |
| `end`           | string | Closing comment delimiter                                                  |
| `after`         | string | Optional regex — header must appear after matching content (e.g. shebangs) |
| `ensure_after`  | string | Optional — content to place after the header block                         |
| `ensure_before` | string | Optional — content to place before the header block                        |

### Available Comment Style IDs

| ID                     | Start  | Middle | End   | Used By                                                                                         |
| ---------------------- | ------ | ------ | ----- | ----------------------------------------------------------------------------------------------- |
| `DoubleSlash`          | `//`   | `//`   | `//`  | Go, Rust, TypeScript, Kotlin, Scala, Java, C/C++, JavaScript, Sass, SCSS, Protocol Buffer, etc. |
| `SlashAsterisk`        | `/*`   | ` *`   | ` */` | Java, C, C++, C#, JavaScript, CSS, Groovy, Kotlin, Swift, Scala, etc.                           |
| `Hashtag`              | `#`    | `#`    | `#`   | Python, Ruby, Shell, YAML, Dockerfile, Makefile, TOML, HCL, Perl, R, etc.                       |
| `AngleBracket`         | `<!--` | `  ~`  | `-->` | HTML, XML, SVG, Vue, Markdown                                                                   |
| `PhpTag`               | `/*`   | ` *`   | ` */` | PHP, HTML+PHP (with `ensure_after: '<?php'` / `ensure_before: '?>'`)                            |
| `Remark`               | `rem`  | `rem`  | `rem` | Batchfile                                                                                       |
| `CurlyBracketDash`     | `{-`   | `~`    | `-}`  | Haskell                                                                                         |
| `CurlyBracketHashtag`  | `{#`   | `~`    | `#}`  | Twig                                                                                            |
| `CurlyBracketAsterisk` | `{*`   | `~`    | `*}`  | Smarty                                                                                          |
| `DoubleDash`           | `--`   | `--`   | `--`  | Lua, SQL, PL/pgSQL, TSQL                                                                        |
| `RoundBracketAsterisk` | `(*`   | `(*`   | `(*`  | OCaml                                                                                           |
| `Semicolon`            | `;`    | `;`    | `;`   | INI                                                                                             |
| `Percent`              | `%`    | `%`    | `%`   | MATLAB                                                                                          |
| `Apostrophe`           | `'`    | `'`    | `'`   | PlantUML, VIM                                                                                   |
| `DoubleDot`            | `..`   | `..`   | `..`  | Racket                                                                                          |
| `Quotes`               | `"`    | `"`    | `"`   | VIM script                                                                                      |
| `PythonStyle`          | `#`    | `#`    | `#`   | Python (handles shebang/encoding lines)                                                         |
| `PythonDocStringStyle` | `"""`  | `~`    | `"""` | Python docstrings                                                                               |

---

## Supported Languages (71)

The tool has built-in support for the following languages with automatic file extension and comment style detection:

AsciiDoc, Batchfile, C, C#, C++, CMake, CoffeeScript, CSS, Cython, Dockerfile, EditorConfig, Elixir, Git Config, Go, Gradle, GraphQL, Groovy, Haskell, HCL, HTML, HTML+PHP, Ignore List, INI, Inko, Inno Setup, Java, Java Properties, JavaScript, JavaScript+ERB, JSON with Comments, JSON5, Kotlin, Lua, M4Sugar, Makefile, Markdown, MATLAB, Nextflow, OCaml, Open Policy Agent, Perl, PHP, Pkl, PlantUML, PLpgSQL, PLSQL, Protocol Buffer, Python, R, Racket, RenderScript, Rust, Sass, Scala, SCSS, Smarty, SQL, SQLPL, Stylus, SVG, Swift, Tcl, TOML, TSQL, TSX, Twig, TypeScript, TXL, VIM script, Vue, XML Property List, YAML

---

## Dependency Section

The `dependency` section configures checking of third-party dependency licenses for compatibility.

```yaml
dependency:
  files:
    - go.mod
  licenses:
    - name: github.com/foo/bar
      version: v1.0.0
      license: MIT
  excludes:
    - name: github.com/internal/pkg
  threshold: 75
  require_fsf_free: false
  require_osi_approved: false
```

### `dependency.files`

List of dependency manifest files to scan. Paths are relative to the `.licenserc.yaml` location.

| File           | Ecosystem      |
| -------------- | -------------- |
| `go.mod`       | Go modules     |
| `pom.xml`      | Maven (Java)   |
| `package.json` | NPM (Node.js)  |
| `Cargo.toml`   | Cargo (Rust)   |
| `Gemfile.lock` | Bundler (Ruby) |

**Ruby-specific behavior:**

- If a `.gemspec` exists alongside `Gemfile.lock`: treated as a library (dev dependencies excluded)
- If no `.gemspec`: treated as an application (all dependencies included)
- `Gemfile.lock` must be committed to version control

### `dependency.licenses`

Manually declare licenses for dependencies that cannot be automatically identified.

| Field     | Type   | Description                                                                                                             |
| --------- | ------ | ----------------------------------------------------------------------------------------------------------------------- |
| `name`    | string | Dependency identifier. Format varies by ecosystem: package path (Go), `GroupID:ArtifactID` (Maven), package name (NPM). |
| `version` | string | Comma-separated versions (e.g. `"1.0,2.0,3.0"`). Empty matches all versions.                                            |
| `license` | string | SPDX license identifier                                                                                                 |

### `dependency.excludes`

Dependencies to exclude from license analysis.

| Field       | Type    | Description                                                                                                  |
| ----------- | ------- | ------------------------------------------------------------------------------------------------------------ |
| `name`      | string  | Dependency identifier                                                                                        |
| `version`   | string  | Specific version(s) to exclude. Empty excludes all versions.                                                 |
| `recursive` | boolean | Exclude transitive dependencies too (Maven only). Non-compile-scope dependencies are automatically excluded. |

### `dependency.threshold`

- **Type:** integer
- **Default:** `75`

Minimum percentage of a file's content that must contain license text for the license to be identified.

### `dependency.require_fsf_free`

- **Type:** boolean
- **Default:** `false`

When `true`, only FSF Free/Libre-marked licenses are considered compatible.

### `dependency.require_osi_approved`

- **Type:** boolean
- **Default:** `false`

When `true`, only OSI-approved licenses are considered compatible.

---

## Config Loading Behavior

1. Tool looks for config at the specified path (default: `.licenserc.yaml`)
2. If the file is missing, built-in defaults are used
3. V2 format (list of header configs) is tried first, then V1 (single object)
4. All relative paths in `dependency.files` are resolved relative to the config file location

---

## Complete Example

```yaml
header:
  license:
    spdx-id: Apache-2.0
    copyright-owner: Apache Software Foundation
    copyright-year: "2024"
    content: |
      Licensed to the Apache Software Foundation (ASF) under one
      or more contributor license agreements.  See the NOTICE file
      distributed with this work for additional information
      regarding copyright ownership.  The ASF licenses this file
      to you under the Apache License, Version 2.0 (the
      "License"); you may not use this file except in compliance
      with the License.  You may obtain a copy of the License at

          http://www.apache.org/licenses/LICENSE-2.0

      Unless required by applicable law or agreed to in writing,
      software distributed under the License is distributed on an
      "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
      KIND, either express or implied.  See the License for the
      specific language governing permissions and limitations
      under the License.
    pattern: |
      Licensed to the Apache Software Foundation.*
      under the Apache License, Version 2.0.*

  paths:
    - "**"

  paths-ignore:
    - "dist"
    - "licenses"
    - "**/*.md"
    - "**/testdata/**"
    - "LICENSE"
    - "NOTICE"

  comment: on-failure
  license-location-threshold: 80

  language:
    Go:
      extensions:
        - ".go"
      comment_style_id: DoubleSlash

dependency:
  files:
    - go.mod
  licenses:
    - name: github.com/some/package
      version: v1.1.10
      license: MIT
  excludes:
    - name: github.com/internal/tool
      recursive: true
  threshold: 75
```
