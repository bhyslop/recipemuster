# jjc-scout

Search across heats and paces with regex patterns. Case-insensitive, searches pace silks, dockets, warrants, and paddock content.

## Usage

```bash
/jjc-scout <pattern> [--actionable]
```

## Arguments

- `<pattern>` - Regex pattern to search for (case insensitive)
- `--actionable` - Optional flag to limit results to rough/bridled paces only

## Examples

```bash
# Search for "implement" across all paces
/jjc-scout implement

# Search for "scout" in actionable paces only
/jjc-scout scout --actionable

# Search with regex pattern
/jjc-scout "jjx_\w+"
```

## Output Format

Results are grouped by heat:

```
₣AF jjk-post-alpha-polish
  ₢AFAAI [rough] implement-scout-search
    docket: ...keyword... for searching across heats
₣AA garlanded-vok-fresh-install
  ₢AAABC [complete] related-pace-name
    paddock: ...keyword... mentioned in context
```

Each matching pace shows:
- Heat firemark and silks (group header)
- Pace coronet, state, and silks
- Field name and excerpt with match context (~60 chars)

Only the first match per pace is shown (keeps output scannable).

## What Gets Searched

Searches in order (first match wins):
1. Pace silks
2. Pace docket (tack text)
3. Pace warrant (if present)
4. Paddock file content

**Note:** Steeplechase/chalk descriptions are NOT searched - scout searches plan artifacts only.

## Exit Status

- 0 - Success
- 1 - Error (invalid pattern, file not found, etc.)

## Implementation

Invokes: `./tt/vvw-r.RunVVX.sh jjx_search <pattern> [--actionable]`
