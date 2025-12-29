# VS Code Extension Testing Guide

## 🕉️ Jagannath Language Extension - Test Instructions

### Prerequisites

1. VS Code 1.85.0 or higher
2. Node.js 18+ installed
3. Extension compiled (`npm run compile`)

### Quick Start Testing

#### Option 1: F5 Development Mode

1. Open the workspace: `d:\fullstack\jagannath\vscode-extension`
2. Press **F5** to launch Extension Development Host
3. In the new VS Code window, open `examples/extension_test.jag`
4. Verify all features below

#### Option 2: Package and Install

```powershell
cd d:\fullstack\jagannath\vscode-extension
npm install -g @vscode/vsce
vsce package
code --install-extension jagannath-1.0.0.vsix
```

---

## 🎨 Feature Verification Checklist

### 1. Syntax Highlighting (TextMate Grammar)

Open `extension_test.jag` and verify these elements have distinct colors:

| Element | Expected Color | Example |
|---------|---------------|---------|
| Keywords | Coral (#ff7b72) | `kāryakrama`, `māna`, `yad`, `phera` |
| Types | Green (#7ee787) | `Saṅkhyā`, `Sūtra`, `Tarka` |
| Functions | Purple (#d2a8ff) | `mukhya`, `kosha_pradarśana` |
| Strings | Light blue (#a5d6ff) | `"जगन्नाथ"` |
| Numbers | Blue (#79c0ff) | `42`, `0x2A`, `3.14159` |
| Comments | Gray (#8b949e) | `// comment`, `/* block */` |
| Operators | Red (#f85149) | `+`, `-`, `*`, `&&`, `||` |

### 2. Semantic Tokens (31 Custom Types)

Hover over these to verify semantic highlighting:

| Token Type | Color | Examples |
|------------|-------|----------|
| Astras | Purple (#a371f7) | `Brahmastra`, `Agneyastra`, `Varunastra` |
| Narakas | Red (#f85149) | `Tamisram`, `Suchimukha`, `Vaitarani` |
| Guṇas | Varies | `sattva` (green), `rajas` (orange), `tamas` (red) |
| Kośas | Blue (#58a6ff) | `anna`, `prāṇa`, `mano`, `vijñāna`, `ānanda` |
| Chakras | Light blue (#79c0ff) | `mūlādhāra`, `anāhata`, `sahasrāra` |
| Varṇas | Orange (#ffa657) | `brāhmaṇa`, `kṣatriya`, `vaiśya`, `śūdra` |
| Mārgas | Light purple (#d2a8ff) | `karma`, `jñāna`, `bhakti`, `rāja` |
| Kārakas | Green (#7ee787) | `kartṛ`, `karman`, `karaṇa`, `sampradāna` |
| Mantras | Orange (#ffa657) | `oṁ`, `namaḥ`, `svāhā`, `phaṭ` |
| Lifetimes | Coral (#ff7b72) | `^1`, `^2`, `sūtra^1` |
| Affixes | Light blue (#79c0ff) | `-ā`, `-l`, `-h`, `-k`, `-b` |

### 3. Hover Information (Sanskrit Glossary)

Hover over these terms and verify tooltip appears:

- `kāryakrama` → Shows "कार्यक्रम - function/procedure"
- `māna` → Shows "मान - variable declaration"
- `Brahmastra` → Shows "ब्रह्मास्त्र - Ultimate weapon - Dead code elimination"
- `Tamisram` → Shows "तामिस्रम् - Hell of darkness - Use-after-free errors"
- `-ā` → Shows "आ - Mutable marker"
- `sattva` → Shows "सत्त्व - Guṇa: Pure quality"

### 4. CodeLens Annotations

Look for clickable annotations above:

- **Functions**: `🕉️ Analyze Function` button above `kāryakrama` definitions
- **Types**: `📊 View Memory Layout` button above `prakāra` definitions
- **Astras**: `⚔️ View Optimization` button where Brahmastra/Agneyastra mentioned
- **Narakas**: `🔥 View Error Classification` where Naraka errors mentioned

### 5. Commands (Ctrl+Shift+P)

Test these commands:

| Command | Action |
|---------|--------|
| `Jagannath: Build` | Runs `jagannath build` in terminal |
| `Jagannath: Run` | Runs `jagannath run` in terminal |
| `Jagannath: Format` | Formats current file |
| `Jagannath: Show AST Viewer` | Opens WebView panel |
| `Jagannath: Show Compilation Pipeline` | Opens pipeline WebView |

### 6. WebView Panels

#### AST Viewer (`Jagannath: Show AST Viewer`)
- Dark theme background (#0d1117)
- Om symbol header (🕉️)
- Expand/Collapse buttons
- Node types colored by category
- Kośa indicators on nodes

#### Pipeline Viewer (`Jagannath: Show Compilation Pipeline`)
- 7-stage pipeline visualization
- Sāṃkhya tattva mapping
- Color-coded stages (1-7)
- Stats placeholders for each stage
- Legend with philosophical terms

### 7. Snippets (Type and Tab)

Test IntelliSense snippets:

| Prefix | Inserts |
|--------|---------|
| `fn` | Function template |
| `struct` | Struct template |
| `if` | Conditional template |
| `loop` | Loop template |
| `match` | Match expression |
| `astra-brahma` | Brahmastra optimization block |
| `naraka-handle` | Error handling with Narakas |
| `varna-brahmin` | Ring 0 function |
| `marga-karma` | Karma path optimization |
| `purushartha` | Optimization goal annotation |

### 8. File Icons

- `.jag` files should show custom icon
- Dark/light theme variants should work

---

## 🐛 Troubleshooting

### Extension Not Loading
```powershell
# Check output panel
View -> Output -> Select "Jagannath Language Server"

# Rebuild extension
cd d:\fullstack\jagannath\vscode-extension
npm run compile
```

### Colors Not Showing
1. Check `settings.json` for conflicting theme settings
2. Try: `Developer: Reload Window`
3. Verify `editor.semanticHighlighting.enabled: true`

### Hover Not Working
1. Ensure file has `.jag` or `.jagannath` extension
2. Check language mode in status bar shows "Jagannath"

---

## 📊 Test Results Template

```
Date: ___________
VS Code Version: ___________
Extension Version: 1.0.0

[ ] Syntax Highlighting
    [ ] Keywords colored coral
    [ ] Types colored green
    [ ] Strings colored light blue
    [ ] Numbers colored blue

[ ] Semantic Tokens
    [ ] Astras colored purple
    [ ] Narakas colored red
    [ ] Guṇas show distinct colors
    [ ] Kośas colored blue

[ ] Hover Information
    [ ] Sanskrit terms show glossary
    [ ] Affixes explained
    [ ] Philosophical concepts documented

[ ] CodeLens
    [ ] Function annotations appear
    [ ] Clickable buttons work

[ ] WebView Panels
    [ ] AST Viewer opens
    [ ] Pipeline Viewer opens
    [ ] Dark theme consistent

[ ] Snippets
    [ ] fn snippet works
    [ ] astra snippets expand correctly

Notes:
_________________________________
_________________________________
```

---

## 🕉️ Om Tat Sat

The extension embodies the philosophical framework of Jagannath:
- **Sattva** colors guide you toward correctness
- **Rajas** colors indicate action and activity
- **Tamas** colors warn of errors and dangers
- **Brahman** colors represent the divine optimizations

May your code achieve **Mokṣa** - liberation through perfect compilation! 🙏
